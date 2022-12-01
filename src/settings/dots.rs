use crate::dots::DotVar;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represent a link between a `source` dotfile in the user defined dotfiles directory
/// and the XDG `target` path where it should be linked
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Dot {
    /// Path relative to user defined dotfile
    pub source: PathBuf,
    /// Target path either relative to $HOME or absolute
    pub target: PathBuf,
    /// Glob pattern of files to ignore when creating symlinks
    #[serde(default)]
    #[serde(skip_serializing)]
    pub ignore: Vec<String>,
    /// A single var file attached to the dot
    #[serde(default = "Dot::default_vars")]
    #[serde(skip_serializing)]
    pub vars: PathBuf,
}

/// Same as dot but source and target are optionals
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DotOverride {
    /// Path relative to user defined dotfile
    pub source: Option<PathBuf>,
    /// Target path either relative to $HOME or absolute
    pub target: Option<PathBuf>,
    /// Glob pattern of files to ignore when creating symlinks
    #[serde(default)]
    pub ignore: Vec<String>,
    /// A single var file attached to the dot
    pub vars: Option<PathBuf>,
}
