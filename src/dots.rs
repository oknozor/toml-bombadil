use std::path::PathBuf;
use anyhow::Result;
use dirs::home_dir;

/// Represent a link between a `source` dotfile in the user defined dotfiles directory
/// and the XDG `target` path where it should be linked
#[derive(Debug, Deserialize, Serialize)]
pub struct DotLink {
    /// Path relative to user defined dotfile
    pub source: PathBuf,
    /// Target path either relative to $HOME or absolute
    pub target: PathBuf,
}

impl DotLink {
    pub fn target(&self) -> Result<PathBuf> {
        if self.target.is_absolute() {
            Ok(self.target.clone())
        } else {
            home_dir()
                .map(|home| home.join(&self.target))
                .ok_or(anyhow!("Unable to find dot path : {:?}", &self.target))
        }
    }
}


