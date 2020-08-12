use anyhow::Result;
use colored::*;
use dirs::home_dir;
use std::path::PathBuf;

/// Represent a link between a `source` dotfile in the user defined dotfiles directory
/// and the XDG `target` path where it should be linked
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Dot {
    /// A name is required when defining profile
    pub name: Option<String>,
    /// Path relative to user defined dotfile
    pub source: PathBuf,
    /// Target path either relative to $HOME or absolute
    pub target: PathBuf,
    /// List of profiles to rapidly switch variables or source file
    pub profile: Option<Vec<Profile>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Profile {
    /// Profile name (used in CLI)
    pub name: String,
    /// Either replace vars or use a different source file
    pub switch: ProfileSwitch,
    /// Post update command hook
    pub hook: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ProfileSwitch {
    Vars(PathBuf),
    Source(PathBuf),
}

impl Dot {
    pub fn target(&self) -> Result<PathBuf> {
        if self.target.is_absolute() {
            Ok(self.target.clone())
        } else {
            home_dir()
                .map(|home| home.join(&self.target))
                .ok_or_else(|| {
                    let err = format!("Unable to find dot path : {:?}", &self.target).red();
                    anyhow!(err)
                })
        }
    }

    pub fn get_profile_names(&self) -> Vec<&str> {
        self.profile
            .as_ref()
            .map(|profiles| {
                profiles
                    .iter()
                    .map(|p| p.name.as_str())
                    .collect::<Vec<&str>>()
            })
            .unwrap_or_default()
    }
}
