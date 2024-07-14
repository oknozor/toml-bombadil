use crate::settings::dots::Dot;
use crate::settings::dots::DotOverride;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// The default profile, containing dot entries, vars and hooks
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ActiveProfile {
    /// A list of symlink to edit
    #[serde(default)]
    pub dots: HashMap<String, Dot>,

    /// Post install hook commands
    #[serde(default)]
    pub prehooks: Vec<String>,

    /// Post install hook commands
    #[serde(default)]
    pub posthooks: Vec<String>,

    /// Variables to use in templates
    #[serde(default)]
    pub vars: Vec<PathBuf>,

    /// Hooks are executed in dotfiles directory
    #[serde(default)]
    pub run_hooks_in_dotfiles_dir: bool,
}

/// An named profile meant to override the default one
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Profile {
    /// A list of symlink to edit
    #[serde(default)]
    pub dots: HashMap<String, DotOverride>,

    /// A list of additional profiles to enable
    #[serde(default)]
    pub extra_profiles: Vec<String>,

    /// Pre install hook commands
    #[serde(default)]
    pub prehooks: Vec<String>,

    /// Post install hook commands
    #[serde(default)]
    pub posthooks: Vec<String>,

    /// Variables to use in templates
    #[serde(default)]
    pub vars: Vec<PathBuf>,

    /// Hooks are executed in dotfiles directory
    #[serde(default)]
    pub run_hooks_in_dotfiles_dir: bool,
}
