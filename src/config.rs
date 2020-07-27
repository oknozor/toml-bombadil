use crate::theming::{Theme, ThemeConfig};
use anyhow::Result;
use config::{Config, ConfigError, File};
use std::fs;
use std::path::{PathBuf};

/// The Global bombadil configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    /// User define dotfiles directory, usually your versionned dotfiles
    pub(crate) dotfiles_dir: String,
    /// A list of symlink to edit
    pub(crate) dot: Vec<DotLink>,
    /// A global theme with optional addons
    pub theme: Option<ThemeConfig>,
}

/// Represent a link between a `source` dotfile in the user defined dotfiles directory
/// and the XDG `target` path where it should be linked
#[derive(Debug, Deserialize, Serialize)]
pub struct DotLink {
    /// User defined dotfile
    pub source: String,
    /// XDG path for the symlink
    pub target: String,
}

impl DotLink {
    // FIXME : needs lazy statics settings here
    /// Resolve dot source path against user defined dotfile directory
    pub(crate) fn dot_source_path(&self) -> Result<PathBuf> {
        let settings = Settings::get()?;
        Ok(settings.bombadil_dots_user_path()?.join(&self.source))
    }

    /// Resolve dot target path against $HOME
    pub(crate) fn xdg_target_path(&self) -> Result<PathBuf> {
        dirs::home_dir().map(|home| home.join(&self.target))
            .ok_or(anyhow!("Unable to find $HOME directory"))
    }

    /// Link a dot entry against user home directory target path
    pub(crate) fn link(&self) -> Result<()> {
        let target_path = PathBuf::from(&self.target);

        if target_path.is_absolute() {
            Ok(std::os::unix::fs::symlink(&self.dot_source_path()?, &target_path)?)
        } else {
            Ok(std::os::unix::fs::symlink(&self.dot_source_path()?, &self.xdg_target_path()?)?)
        }
    }
}

impl Settings {
    /// Try to find a theme matching `[theme.name] config property`
    /// Fallback to the default `Theme` impl is none is found
    pub fn load_theme(&self) -> Theme {
        let theme_settings = &self
            .theme
            .as_ref();

        match theme_settings {
            Some(theme_settings) => match theme_settings.name.as_ref() {
                Some(theme_name) => match resolve_theme(theme_name) {
                    Ok(theme) => theme,
                    Err(err) => {
                        eprintln!("{}. Falling back to default theme", err);
                        Theme::default()
                    }
                }
                None => {
                    eprintln!("[theme.name] is not set in bombadil.toml falling back to default theme");
                    Theme::default()
                }
            },
            None => {
                eprintln!("[theme] settings no set in bombadil.toml falling back to default");
                Theme::default()
            }
        }
    }

    /// Resolve bombadil settings against its standard xdg path :
    /// `$XDG_CONFIG_DIR/bombadil/config.toml
    pub fn get() -> Result<Self> {
        match Self::bombadil_config_xdg_path() {
            Ok(path) => {
                if path.exists() {
                    let mut s = Config::new();
                    s.merge(File::from(path))?;
                    s.try_into()
                        .map_err(|err| anyhow!("Config format error : {}", err))
                } else {
                    Err(anyhow!("Unable to find bombadil config file {}", path.display()))
                }
            }
            Err(err) =>  Err(anyhow!("Config error : {}", err))
        }

    }

    /// Resolve `$XDG_CONFIG_DIR`
    pub fn xdg_config_dir() -> Result<PathBuf, ConfigError> {
        dirs::config_dir().ok_or(ConfigError::NotFound(
            "Unable to find XDG_CONFIG_DIR".into(),
        ))
    }

    /// Resole the bombadil XDG config path : `$XDG_CONFIG_DIR/.bombadil`
    pub fn bombadil_xdg_path() -> Result<PathBuf, ConfigError> {
        Ok(Self::xdg_config_dir()?.join(".bombadil"))
    }

    /// Resole the bombadil XDG settings path : `$XDG_CONFIG_DIR/bombadil/config.toml
    pub fn bombadil_config_xdg_path() -> Result<PathBuf, ConfigError> {
        Ok(Self::bombadil_xdg_path()?.join("config.toml"))
    }

    /// Resole the bombadil XDG theme path : `$XDG_CONFIG_DIR/bombadil/themes
    pub fn bombadil_theme_xdg_path() -> Result<PathBuf, ConfigError> {
        Ok(Self::bombadil_xdg_path()?.join("themes"))
    }

    /// Resole the bombadil XDG theme path : `$XDG_CONFIG_DIR/bombadil/themes
    pub fn bombadil_theme_user_path(&self) -> Result<PathBuf, ConfigError> {
        Ok(Self::bombadil_xdg_path()?.join("themes"))
    }

    /// Resole the bombadil user defined path
    pub fn bombadil_dots_user_path(&self) -> Result<PathBuf, ConfigError> {
        dirs::home_dir()
            .ok_or(ConfigError::NotFound("Unable to find $HOME directory".into()))
            .map(|home| home.join(&self.dotfiles_dir))
            .map_err(|_err| ConfigError::NotFound("Unable to find toml setting `dotfile_dir`".into()))
    }

    /// Resole the bombadil path theme path in dotfiles user dir
    pub fn bombadil_dots_user_theme_path(&self) -> Result<PathBuf, ConfigError> {
        dirs::home_dir()
            .ok_or(ConfigError::NotFound("Unable to find $HOME directory".into()))
            .map(|home| home
                .join(&self.dotfiles_dir)
                .join("themes"))
            .map_err(|_err| ConfigError::NotFound("Unable to find themes directory".into()))
    }
}

fn resolve_theme(theme_name: &str) -> Result<Theme> {
    let theme_path = dirs::config_dir()
        .map(|config_dir| config_dir.join(".themes"))
        .map(|theme_dir| theme_dir.join(theme_name))
        .ok_or(anyhow!("Xdg config directory not found, falling back to default theme"))?;

    // unwrapping `theme_path` is safe here
    let content = fs::read_to_string(&theme_path)
        .map_err(|err| anyhow!("Cannot read {}, cause {}", theme_path.to_str().unwrap(), err))?;

    toml::from_str(&content)
        .map_err(|err| anyhow!("Cannot parse theme : {}", err))
}
