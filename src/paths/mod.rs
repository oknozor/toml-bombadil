use crate::error::Error::{
    Backup, Copy, SetPermissions, SourceNotFound, Symlink, TargetNotFound, TemplateNotFound, Unlink,
};
use crate::error::*;
use crate::settings::dotfile_dir;
use crate::{Dot, DotVar};
use anyhow::anyhow;
use dirs::home_dir;
use std::io::{BufRead, BufReader};
use std::os::unix;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::{fs, io};

pub trait DotPaths {
    /// Return the target path of a dot entry either absolute or relative to $HOME
    fn target(&self) -> Result<PathBuf>;

    /// Resolve dot source copy path ({dotfiles/dotsource) against user defined dotfile directory
    /// Check if file exists
    fn source(&self) -> Result<PathBuf>;

    /// Resolve the dotfile rendered template path
    fn copy_path(&self) -> Result<PathBuf>;

    /// Build the rendered template path, use this to create the rendered file
    fn build_copy_path(&self) -> PathBuf;

    /// Remove the dotfile target symlink
    fn unlink(&self) -> Result<()>;

    /// Symlink the dotfile to its destination
    fn symlink(&self) -> Result<()>;

    fn resolve_var_path(&self) -> Option<PathBuf>;

    fn hard_copy_target(&self) -> Option<Result<PathBuf>>;
}

impl DotPaths for Dot {
    fn target(&self) -> Result<PathBuf> {
        if self.target.is_absolute() {
            Ok(self.target.clone())
        } else {
            home_dir()
                .map(|home| home.join(&self.target))
                .ok_or_else(|| TargetNotFound(self.target.clone()))
        }
    }

    fn hard_copy_target(&self) -> Option<Result<PathBuf>> {
        self.hard_copy_target.as_ref()?;
        let hard_copy_target = self.hard_copy_target.clone().unwrap();
        if hard_copy_target.is_absolute() {
            Some(Ok(hard_copy_target.clone()))
        } else {
            Some(
                home_dir()
                    .map(|home| home.join(hard_copy_target.clone()))
                    .ok_or_else(|| TargetNotFound(hard_copy_target.clone())),
            )
        }
    }

    fn source(&self) -> Result<PathBuf> {
        let path = dotfile_dir().join(&self.source);

        if path.exists() {
            Ok(path)
        } else {
            Err(SourceNotFound(path))
        }
    }

    fn copy_path(&self) -> Result<PathBuf> {
        let path = self.build_copy_path();

        path.canonicalize()
            .map_err(|error| TemplateNotFound { path, error })
    }

    fn build_copy_path(&self) -> PathBuf {
        dotfile_dir().join(".dots").join(&self.source)
    }

    fn unlink(&self) -> Result<()> {
        unlink(&self.target)
    }

    fn symlink(&self) -> Result<()> {
        let copy_path = &self.copy_path()?;
        let target = &self.target()?;
        let hard_copy_target = match self.hard_copy_target() {
            Some(Ok(path)) => Some(path),
            Some(Err(cause)) => return Err(cause),
            None => None,
        };

        let symlink_exists = if let Ok(target) = target.canonicalize() {
            &target == copy_path
        } else {
            false
        };
        let hard_copy_exists = match hard_copy_target.clone() {
            Some(path) => path.exists(),
            None => false,
        };
        let hard_copy_permissions_match = match self.hard_copy_permissions {
            None => true,
            Some(target_permissions) => {
                if let Some(hard_copy_target) = hard_copy_target.clone() {
                    if hard_copy_target.exists() {
                        let metadata = hard_copy_target.metadata().unwrap();
                        let permissions = metadata.permissions().mode();
                        permissions & 0o777 == target_permissions
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        };

        if let Some(parent) = target.parent() {
            if !parent.exists() {
                println!("Creating parent: {:?}", parent);
                let create_fs_err = fs::create_dir_all(parent);
                match create_fs_err {
                    Ok(_) => Ok(()),
                    Err(cause) => match cause.kind() {
                        std::io::ErrorKind::PermissionDenied => symlink_as_sudo(self),
                        _ => Err(Symlink {
                            source_path: copy_path.to_owned(),
                            target: target.to_owned(),
                            cause,
                        }),
                    },
                }?;
            }
        }

        // Link
        if !symlink_exists {
            let symlink_err = unix::fs::symlink(copy_path, target);
            match symlink_err {
                Ok(_) => Ok(()),
                Err(cause) => match cause.kind() {
                    std::io::ErrorKind::AlreadyExists => {
                        move_original_to_backup(target)?;
                        self.symlink()
                    }
                    std::io::ErrorKind::PermissionDenied => symlink_as_sudo(self),
                    _ => Err(Symlink {
                        source_path: copy_path.to_owned(),
                        target: target.to_owned(),
                        cause,
                    }),
                },
            }?;
        }

        // Make hard copy if needed.
        if !hard_copy_exists {
            if let Some(hard_copy_target) = hard_copy_target.clone() {
                println!(
                    "Making hard copy for file: {:?} -> {:?}",
                    target, hard_copy_target
                );
                copy(target, hard_copy_target.as_path())?;
            }
        }

        // Set hard copy permissions if needed.
        if !hard_copy_permissions_match {
            if let Some(hard_copy_target) = hard_copy_target.clone() {
                if let Some(hard_copy_permissions) = self.hard_copy_permissions {
                    println!(
                        "Setting permissions for hard copy: {:?} -> {:o}",
                        hard_copy_target, hard_copy_permissions
                    );
                    set_permissions(hard_copy_target.as_path(), hard_copy_permissions)?;
                }
            }
        }

        Ok(())
    }

    fn resolve_var_path(&self) -> Option<PathBuf> {
        self.resolve_from_source(&self.source, &self.vars)
    }
}

fn move_original_to_backup(target: &Path) -> Result<()> {
    let target_as_non_absolute = if target.is_absolute() {
        target.strip_prefix("/").unwrap()
    } else {
        target
    };
    let backup_path = dotfile_dir().join(".backups").join(target_as_non_absolute);
    println!(
        "Backing up original file: {:?} to {:?}",
        target, backup_path
    );

    if let Some(parent) = backup_path.parent() {
        if !parent.exists() {
            println!("Creating backup parent: {:?}", parent);
            fs::create_dir_all(parent).map_err(|cause| Backup {
                target: target.to_path_buf(),
                backup_path: backup_path.clone(),
                cause: anyhow!("Failed to create backup parent: {:?}", cause),
            })?;
        }
    }

    println!(
        "Copying original file to backup: {:?} -> {:?}",
        target, backup_path
    );
    copy(target, &backup_path).map_err(|cause| Backup {
        target: target.to_path_buf(),
        backup_path,
        cause: cause.into(),
    })?;

    println!("Deleting original file: {:?}", target);
    unlink(target)?;

    Ok(())
}

fn copy(from: &Path, to: &Path) -> Result<()> {
    if let Some(parent) = to.parent() {
        if !parent.exists() {
            println!("Creating copy target parent: {:?}", parent);
            fs::create_dir_all(parent).map_err(|cause| Copy {
                from: from.to_path_buf(),
                to: to.to_path_buf(),
                cause: anyhow!("Failed to create copy target parent: {:?}", cause),
            })?;
        }
    }

    println!("Copying file: {:?} -> {:?}", from, to);
    match fs::copy(from, to) {
        Ok(_) => (),
        Err(cause) => match cause.kind() {
            std::io::ErrorKind::PermissionDenied => {
                println!("Copying file (as sudo): {:?} -> {:?}", from, to);
                let status = run_cmd(
                    format!("copy `{:?} -> {:?}`", from, to),
                    Command::new("sudo").arg("cp").arg("-R").arg(from).arg(to),
                )?;
                if !status.success() {
                    return Err(Copy {
                        from: from.to_path_buf(),
                        to: to.to_path_buf(),
                        cause: anyhow!("Failed to copy (as sudo)"),
                    });
                }
            }
            _ => {
                return Err(Copy {
                    from: from.to_path_buf(),
                    to: to.to_path_buf(),
                    cause: anyhow!("Failed to copy"),
                })
            }
        },
    };

    Ok(())
}

fn set_permissions(path: &Path, permissions: u32) -> Result<()> {
    println!("Setting permissions: {:?} -> {:o}", path, permissions);

    let perm = fs::Permissions::from_mode(permissions);
    match fs::set_permissions(path, perm) {
        Ok(_) => (),
        Err(cause) => match cause.kind() {
            std::io::ErrorKind::PermissionDenied => {
                println!(
                    "Setting permissions (as sudo): {:?} -> {:o}",
                    path, permissions
                );
                let status = run_cmd(
                    format!("chmod `{:o} {:?}`", permissions, path),
                    Command::new("sudo")
                        .arg("chmod")
                        .arg(format!("{:o}", permissions))
                        .arg(path),
                )?;
                if !status.success() {
                    return Err(SetPermissions {
                        path: path.to_path_buf(),
                        permissions,
                        cause: anyhow!("Failed to set permissions (as sudo)"),
                    });
                }
            }
            _ => {
                return Err(SetPermissions {
                    path: path.to_path_buf(),
                    permissions,
                    cause: cause.into(),
                });
            }
        },
    };

    Ok(())
}

fn symlink_as_sudo(dot: &Dot) -> Result<()> {
    let copy_path = &dot.copy_path()?;
    let target = &dot.target()?;

    if let Ok(target) = target.canonicalize() {
        if &target == copy_path {
            return Ok(());
        }
    }

    if let Some(parent) = target.parent() {
        println!("Creating parent (as sudo): {:?}", parent);
        let status = run_cmd(
            format!("mkdir `{:?}`", parent),
            Command::new("sudo").arg("mkdir").arg("-p").arg(parent),
        )?;

        if !status.success() {
            return Err(Symlink {
                source_path: copy_path.to_owned(),
                target: target.to_owned(),
                cause: std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to create parent (as sudo)",
                ),
            });
        }
    }

    println!(
        "Creating symlink (as sudo): {:?} -> {:?}",
        copy_path, target
    );
    let status = run_cmd(
        format!("link `{:?} -> {:?}`", copy_path, target),
        Command::new("sudo")
            .arg("ln")
            .arg("-s")
            .arg(copy_path)
            .arg(target),
    )?;

    if !status.success() {
        return Err(Symlink {
            source_path: copy_path.to_owned(),
            target: target.to_owned(),
            cause: std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to create symlink (as sudo)",
            ),
        });
    }

    Ok(())
}

pub fn unlink<P: AsRef<Path> + ?Sized>(path: &P) -> Result<()> {
    let unlink_err = unlink_safe(path);
    match unlink_err {
        Ok(_) => Ok(()),
        Err(cause) => match cause.kind() {
            std::io::ErrorKind::PermissionDenied => Err(Unlink {
                error: cause,
                path: path.as_ref().to_owned(),
            }),
            _ => unlink_sudo(path),
        },
    }
}

fn unlink_safe<P: AsRef<Path> + ?Sized>(path: &P) -> std::io::Result<()> {
    if fs::symlink_metadata(path).is_ok() {
        if path.as_ref().is_dir() {
            return fs::remove_dir_all(path);
        }
        return fs::remove_file(path);
    }

    Ok(())
}

fn unlink_sudo<P: AsRef<Path> + ?Sized>(path: &P) -> Result<()> {
    if fs::symlink_metadata(path).is_err() {
        return Ok(());
    }

    println!("Deleting symlink (as sudo): {:?}", path.as_ref());
    let status = run_cmd(
        format!("unlink `{}`", path.as_ref().display()),
        Command::new("sudo").arg("rm").arg("-rf").arg(path.as_ref()),
    )?;
    if !status.success() {
        return Err(Unlink {
            error: std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to delete symlink (as sudo)",
            ),
            path: path.as_ref().to_path_buf(),
        });
    }

    Ok(())
}

fn run_cmd(log_prefix: String, cmd: &mut Command) -> io::Result<ExitStatus> {
    let mut child = cmd.stderr(Stdio::piped()).stdout(Stdio::piped()).spawn()?;

    BufReader::new(child.stdout.take().unwrap())
        .lines()
        .for_each(|line| println!("[{}] {}", log_prefix, line.unwrap_or_else(|_| "".into())));

    BufReader::new(child.stderr.take().unwrap())
        .lines()
        .for_each(|line| eprintln!("[{}] {}", log_prefix, line.unwrap_or_else(|_| "".into())));

    child.wait()
}
