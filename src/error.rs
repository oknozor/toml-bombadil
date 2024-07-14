use std::io;
use std::path::PathBuf;
use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to find target path : {0}")]
    TargetNotFound(PathBuf),

    #[error("Dotfile source path {0} does not exist")]
    SourceNotFound(PathBuf),

    #[error("IoError: {0}")]
    Io(#[from] io::Error),

    #[error("Rendered template not found {path}, cause: {error}")]
    TemplateNotFound { path: PathBuf, error: io::Error },

    #[error("Failed to unlink dotfile : {path}, cause = {error}")]
    Unlink { path: PathBuf, error: io::Error },

    #[error("Failed to symlink dotfile, source: {source_path}, target: {target}, cause: {cause}")]
    Symlink {
        source_path: PathBuf,
        target: PathBuf,
        cause: io::Error,
    },

    #[error(
        "Failed to backup original file, target: {target}, backup: {backup_path}, cause: {cause}"
    )]
    Backup {
        target: PathBuf,
        backup_path: PathBuf,
        cause: anyhow::Error,
    },

    #[error("Failed to copy file, from: {from}, to: {to}, cause: {cause}")]
    Copy {
        from: PathBuf,
        to: PathBuf,
        cause: anyhow::Error,
    },

    #[error(
        "Failed to set permissions on file: {path}, permissions: {permissions}, cause: {cause}"
    )]
    SetPermissions {
        path: PathBuf,
        permissions: u32,
        cause: anyhow::Error,
    },
}
