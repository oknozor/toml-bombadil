use crate::settings::imports::ImportPath;
use crate::settings::profiles::ActiveProfile;
use crate::{Gpg, Profile, BOMBADIL_CONFIG};
use anyhow::anyhow;
use colored::*;
use config::Config;
use config::{ConfigError, File};
use dirs::home_dir;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub mod dots;
pub mod imports;
pub mod profiles;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::get().unwrap_or_default();
    pub static ref GPG: Option<Gpg> = {
        SETTINGS
            .gpg_user_id
            .as_ref()
            .map(|gpg| Gpg::new(gpg.as_str()))
    };
}

pub fn profiles() -> Vec<&'static str> {
    SETTINGS
        .profiles
        .keys()
        .map(|profile| profile.as_ref())
        .collect()
}

pub fn dotfile_dir() -> PathBuf {
    home_dir()
        .expect("$HOME should be set")
        .join(&SETTINGS.dotfiles_dir)
}

/// The Global bombadil configuration
#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    /// User define dotfiles directory, usually your versioned dotfiles
    pub dotfiles_dir: PathBuf,

    pub gpg_user_id: Option<String>,

    #[serde(default)]
    pub verbosity: bool,

    #[serde(default)]
    pub settings: ActiveProfile,

    #[serde(default)]
    pub profiles: HashMap<String, Profile>,

    /// Paths to merge with the main configuration
    #[serde(default)]
    pub import: Vec<ImportPath>,
}

impl Settings {
    /// Resolve bombadil settings against its standard xdg path :
    /// `$XDG_CONFIG_DIR/bombadil.toml`
    pub fn get() -> anyhow::Result<Self> {
        match Self::bombadil_config_xdg_path() {
            Ok(path) => {
                if path.exists() {
                    let mut settings = Config::builder()
                        .add_source(File::from(path))
                        .build()?
                        .try_deserialize::<Settings>()
                        .map_err(|err| anyhow!("{} : {}", "Config format error".red(), err));

                    if let Ok(settings) = settings.as_mut() {
                        settings.merge_imports()?;
                    }

                    settings
                } else {
                    Err(anyhow!(
                        "Unable to find bombadil settings file {}",
                        path.display()
                    ))
                }
            }
            Err(err) => Err(anyhow!("Config error : {}", err)),
        }
    }

    /// Resolve the bombadil XDG settings path : `$XDG_CONFIG_DIR/bombadil.toml
    pub fn bombadil_config_xdg_path() -> anyhow::Result<PathBuf, ConfigError> {
        dirs::config_dir()
            .ok_or_else(|| {
                ConfigError::NotFound("Unable to find `$XDG_CONFIG/bombadil.toml`".into())
            })
            .map(|path| path.join(BOMBADIL_CONFIG))
    }

    pub(crate) fn get_dotfiles_path(&self) -> anyhow::Result<PathBuf> {
        let home_dir = dirs::home_dir();
        if home_dir.is_none() {
            return Err(anyhow!("$HOME directory not found"));
        }

        let path = if self.dotfiles_dir.is_absolute() {
            self.dotfiles_dir.to_owned()
        } else {
            home_dir.unwrap().join(&self.dotfiles_dir)
        };

        if !path.exists() {
            return Err(anyhow!("Dotfiles directory {:?} does not exist", &path));
        }

        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use crate::settings::Settings;
    use speculoos::prelude::*;

    #[test]
    fn should_get_bombadil_path() {
        let path = Settings::bombadil_config_xdg_path();
        assert_that!(path).is_ok();
    }
}
