use crate::error::Error::{SourceNotFound, Symlink, TargetNotFound, TemplateNotFound, Unlink};
use crate::error::*;
use crate::settings::dotfile_dir;
use crate::{Dot, DotVar};
use dirs::home_dir;
use std::fs;
use std::os::unix;
use std::path::{Path, PathBuf};

pub trait DotPaths {
    /// Return the target path of a dot entry either absolute or relative to $HOME
    fn target(&self) -> Result<PathBuf>;

    /// Resolve dot source copy path ({dotfiles}/dotsource) against user defined dotfile directory
    /// Check if file exists
    fn source(&self) -> Result<PathBuf>;

    /// Resolve the dotfile rendered template path
    fn copy_path(&self) -> Result<PathBuf>;

    /// Build the rendered template path, use this to create the rendered file
    fn copy_path_unchecked(&self) -> PathBuf;

    /// Remove the dotfile target symlink
    fn unlink(&self) -> Result<()>;

    /// Symlink the dotfile to its destination
    fn symlink(&self, force: bool) -> Result<()>;

    /// Symlink the dotfile to its destination directly with no templating
    fn symlink_direct(&self, force: bool) -> Result<()>;

    fn resolve_var_path(&self) -> Option<PathBuf>;
}

impl DotPaths for Dot {
    fn target(&self) -> Result<PathBuf> {
        let path = &self.target.to_string_lossy();
        let path = shellexpand::tilde(path.as_ref());
        let path = Path::new(path.as_ref());

        if path.is_absolute() {
            Ok(path.to_path_buf())
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
        let path = self.copy_path_unchecked();
        let path = path.to_string_lossy();
        let path = shellexpand::tilde(path.as_ref());
        let path = Path::new(path.as_ref());
        path.canonicalize().map_err(|error| TemplateNotFound {
            path: path.into(),
            error,
        })
    }

    fn copy_path_unchecked(&self) -> PathBuf {
        dotfile_dir().join(".dots").join(&self.source)
    }

    fn unlink(&self) -> Result<()> {
        unlink(&self.target)
    }

    fn symlink_direct(&self, force: bool) -> Result<()> {
        let source = &self.source()?;
        let target = &self.target()?;
        let path = target.to_string_lossy();
        let path = shellexpand::tilde(path.as_ref());
        let target = Path::new(path.as_ref());

        if let Some(p) = target.parent() {
            fs::create_dir_all(p).ok();
        }

        if target.exists() && !target.metadata()?.is_symlink() && force {
            let backup = target.with_extension("bak");
            if backup.exists() && backup.is_file() {
                fs::remove_file(&backup)?;
            }
            if backup.exists() && backup.is_dir() {
                fs::remove_dir_all(&backup)?;
            }

            println!("Backing up {} to {}", target.display(), backup.display());
            fs::copy(target, backup)?;
            fs::remove_file(target)?;
        }

        // Link
        unix::fs::symlink(source, target)
            .map_err(|cause| {
                let source_path = self.source.clone();
                let target = self.target.clone();

                Symlink {
                    source_path,
                    target,
                    cause,
                }
            })
            .unwrap_or_else(|err| eprintln!("{:?}", err));

        Ok(())
    }

    fn symlink(&self, force: bool) -> Result<()> {
        let copy_path = &self.copy_path()?;
        let target = &self.target()?;
        let path = target.to_string_lossy();
        let path = shellexpand::tilde(path.as_ref());
        let target = Path::new(path.as_ref());

        if let Ok(target) = target.canonicalize() {
            if &target == copy_path {
                return Ok(());
            }
        }
        if let Some(p) = target.parent() {
            fs::create_dir_all(p).ok();
        }

        if target.exists() && !target.metadata()?.is_symlink() && force {
            let backup = target.with_extension("bak");
            if backup.exists() && backup.is_file() {
                fs::remove_file(&backup)?;
            }
            if backup.exists() && backup.is_dir() {
                fs::remove_dir_all(&backup)?;
            }

            println!("Backing up {} to {}", target.display(), backup.display());
            fs::copy(target, backup)?;
            fs::remove_file(target)?;
        }

        // Link
        unix::fs::symlink(copy_path, target)
            .map_err(|cause| {
                let source_path = self.source.clone();
                let target = self.target.clone();

                Symlink {
                    source_path,
                    target,
                    cause,
                }
            })
            .unwrap_or_else(|err| eprintln!("{:?}", err));

        Ok(())
    }

    fn resolve_var_path(&self) -> Option<PathBuf> {
        self.resolve_from_source(&self.source, &self.vars)
    }
}

pub fn unlink<P: AsRef<Path> + ?Sized>(path: &P) -> Result<()> {
    if fs::symlink_metadata(path).is_ok() {
        if path.as_ref().is_dir() {
            fs::remove_dir_all(path).map_err(|error| Unlink {
                path: path.as_ref().to_path_buf(),
                error,
            })?
        } else {
            fs::remove_file(path).map_err(|error| Unlink {
                path: path.as_ref().to_path_buf(),
                error,
            })?
        }
    }

    Ok(())
}
