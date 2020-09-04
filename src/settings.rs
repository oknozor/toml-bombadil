use crate::dots::{Dot, DotOverride};
use anyhow::Result;
use config::{Config, ConfigError, File};
use std::path::PathBuf;
use colored::Colorize;
use std::collections::HashMap;

/// The Global bombadil configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    /// User define dotfiles directory, usually your versioned dotfiles
    pub(crate) dotfiles_dir: PathBuf,

    #[serde(default)]
    pub settings: ActiveProfile,

    #[serde(default)]
    pub profiles: HashMap<String, Profile>,

    /// Paths to merge with the main configuration
    #[serde(default)]
    pub import: Vec<ImportPath>,
}

/// An imported configuration, same as `Settings` but without `dotfiles_dir`
#[derive(Debug, Deserialize, Serialize)]
pub struct ImportedSettings {
    #[serde(default)]
    pub settings: ActiveProfile,

    #[serde(default)]
    pub profiles: HashMap<String, Profile>,

    /// Paths to merge with the main configuration
    #[serde(default)]
    pub import: Vec<ImportPath>,
}

/// The default profile, containing dot entries, vars and hooks
#[derive(Debug, Deserialize, Serialize)]
pub struct ActiveProfile {
    /// A list of symlink to edit
    #[serde(default)]
    pub dots: HashMap<String, Dot>,

    /// Post install hook commands
    #[serde(default)]
    pub hooks: Vec<String>,

    /// Variables to use in templates
    #[serde(default)]
    pub vars: Vec<PathBuf>,
}

/// An named profile meant to override the default one
#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    /// A list of symlink to edit
    #[serde(default)]
    pub dots: HashMap<String, DotOverride>,

    /// Post install hook commands
    #[serde(default)]
    pub hooks: Vec<String>,

    /// Variables to use in templates
    #[serde(default)]
    pub vars: Vec<PathBuf>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ImportPath {
    path: PathBuf
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Var {
    pub path: PathBuf,
}

impl Default for ActiveProfile {
    fn default() -> Self {
        Self {
            dots: Default::default(),
            hooks: vec![],
            vars: vec![],
        }
    }
}

impl Settings {
    /// Resolve bombadil settings against its standard xdg path :
    /// `$XDG_CONFIG_DIR/bombadil.toml`
    pub fn get() -> Result<Self> {
        match Self::bombadil_config_xdg_path() {
            Ok(path) => {
                if path.exists() {
                    let mut s = Config::new();
                    s.merge(File::from(path))?;
                    let mut settings: Result<Settings> = s.try_into()
                        .map_err(|err| anyhow!("Config format error : {}", err));

                    if let Ok(settings) = settings.as_mut() {
                        settings.merge_imports()?;
                    }

                    settings
                } else {
                    Err(anyhow!(
                        "Unable to find bombadil config file {}",
                        path.display()
                    ))
                }
            }
            Err(err) => Err(anyhow!("Config error : {}", err)),
        }
    }

    fn merge_imports(&mut self) -> Result<()> {
        let import_paths: Vec<PathBuf> = self.import
            .iter()
            .map(|import| import.path.clone())
            .collect();

        for path in import_paths.iter() {
            if path.exists() {
                let mut s = Config::new();
                s.merge(File::from(path.to_owned()))?;

                let sub_setting = s.try_into::<ImportedSettings>()
                    .map_err(|err| anyhow!("Config format error : {}", err));

                match sub_setting {
                    Ok(sub_settings) => self.merge(sub_settings),
                    Err(err) => eprintln!("{} {:?} {}", "Error loading settings from : ", path, err )
                }

            } else {
                eprintln!("{} {}", "Unable to find bombadil import file".red(), path.display());
            }
        }

        Ok(())
    }

    fn merge(&mut self, sub_settings: ImportedSettings) {
        self.settings.hooks.extend_from_slice(&sub_settings.settings.hooks);
        self.settings.vars.extend_from_slice(&sub_settings.settings.vars);
        self.import.extend_from_slice(&sub_settings.import);
        self.settings.dots.extend(sub_settings.settings.dots);
        self.profiles.extend(sub_settings.profiles);
    }

    /// Resolve the bombadil XDG settings path : `$XDG_CONFIG_DIR/bombadil.toml
    pub fn bombadil_config_xdg_path() -> Result<PathBuf, ConfigError> {
        dirs::config_dir()
            .ok_or_else(|| {
                ConfigError::NotFound("Unable to find `$XDG_CONFIG/bombadil.toml`".into())
            })
            .map(|path| path.join("bombadil.toml"))
    }
}

#[cfg(test)]
mod tests {
    use temp_testdir::TempDir;
    use crate::Bombadil;
    use crate::settings::Settings;
    use std::ops::Not;

    #[test]
    fn should_merge_import() {
        // Arrange
        let tmp = TempDir::new("/tmp/import_test", false).to_path_buf();
        std::fs::copy("tests/imports/import.toml", tmp.join("import.toml")).unwrap();
        std::fs::copy("tests/imports/bombadil.toml", tmp.join("bombadil.toml")).unwrap();
        Bombadil::link_self_config(Some(tmp.join("bombadil.toml"))).unwrap();

        // Act
        let settings = Settings::get().unwrap();

        // Assert
        assert_eq!(settings.dotfiles_dir.to_str().unwrap(), "/tmp/import_test");
        assert!(settings.settings.dots.is_empty().not());

        std::fs::remove_dir_all(tmp).unwrap();
    }
}