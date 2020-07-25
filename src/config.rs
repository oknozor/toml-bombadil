use config::{ConfigError, Config, File};
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub dotfiles_dir: String,
    pub dot: Vec<DotLink>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DotLink {
    pub source: String,
    pub target: String,
}

impl Settings {
    pub fn get() -> Result<Self, ConfigError> {
        let path = Self::xdg_path()?;
        if path.exists() {
            let mut s = Config::new();
            s.merge(File::from(path))?;
            s.try_into()
        } else {
            Err(ConfigError::NotFound("Unable to find config file".into()))
        }
    }

    pub fn xdg_path() -> Result<PathBuf, ConfigError> {
        let mut path = dirs::config_dir()
            .ok_or(ConfigError::NotFound("Unable to find XDG_CONFIG_DIR".into()))?;
        path.push(".bombadil.toml");

        Ok(path)
    }
}