use config::{ConfigError, Config, File};
use std::path::{Path, PathBuf};
use crate::color::{ThemeConfig, Theme};
use std::fs;
use anyhow::Result;

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub dotfiles_dir: String,
    pub dot: Vec<DotLink>,
    pub theme: Option<ThemeConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DotLink {
    pub source: String,
    pub target: String,
}

impl Settings {
    pub fn load_theme(&self) -> Theme {

        let theme_name = &self.theme.as_ref()
            .and_then(|theme_settings| theme_settings.name.as_ref());

        if let Some(theme_name) = theme_name {
            let theme_path = dirs::home_dir()
                .map(|home| home.join("themes"))
                .map(|theme_dir| theme_dir.join(theme_name))
                .ok_or(Theme::default());

            if let Ok(theme_path) = theme_path {
                if let Ok(theme_content) = fs::read_to_string(theme_path) {
                    toml::from_str(&theme_content).unwrap()
                } else {
                    Theme::default()
                }
            } else {
                Theme::default()
            }
        } else {
            Theme::default()
        }
    }

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

    pub fn xdg_config_dir() -> Result<PathBuf, ConfigError> {
        dirs::config_dir()
            .ok_or(ConfigError::NotFound("Unable to find XDG_CONFIG_DIR".into()))
    }
    pub fn xdg_path() -> Result<PathBuf, ConfigError> {
        let mut path = Self::xdg_config_dir()?;
        path.push(".bombadil.toml");

        Ok(path)
    }
}