use crate::error::Error::{SourceNotFound, Symlink, TargetNotFound, TemplateNotFound, Unlink};
use crate::error::*;
use crate::settings::dotfile_dir;
use crate::{Dot, DotVar};
use dirs::home_dir;
use std::io::{BufRead, BufReader};
use std::os::unix;
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

        if let Ok(target) = target.canonicalize() {
            if &target == copy_path {
                return Ok(());
            }
        }

        if let Some(parent) = target.parent() {
            println!("Creating parent: {:?}", parent);
            let create_fs_err = fs::create_dir_all(parent);
            match create_fs_err {
                Ok(_) => Ok(()),
                Err(cause) => match cause.kind() == std::io::ErrorKind::PermissionDenied {
                    true => symlink_as_sudo(self),
                    false => Err(Symlink {
                        source_path: copy_path.to_owned(),
                        target: target.to_owned(),
                        cause,
                    }),
                },
            }?;
        }

        // Link
        let symlink_err = unix::fs::symlink(copy_path, target);
        match symlink_err {
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

        Ok(())
    }

    fn resolve_var_path(&self) -> Option<PathBuf> {
        self.resolve_from_source(&self.source, &self.vars)
    }
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
