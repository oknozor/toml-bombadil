use crate::theming::{Theme, ThemeConfig};
use anyhow::Result;
use config::{Config, ConfigError, File};
use std::fs;
use std::path::PathBuf;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::get().unwrap();
}

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
        Ok(SETTINGS.bombadil_dotfile_path()?.join(&self.source))
    }

    /// Resolve dot target path against $HOME or absolute path
    pub(crate) fn target_path(&self) -> Result<PathBuf> {
        let target_path = PathBuf::from(&self.target);

        if target_path.is_absolute() {
            Ok(target_path)
        } else {
            dirs::home_dir()
                .map(|home| home.join(&self.target))
                .ok_or(anyhow!("Unable to find $HOME directory"))
        }
    }

    /// Symlink a dot entry
    pub(crate) fn link(&self) -> Result<()> {
        let source = self.dot_source_path()?;
        let target = self.target_path()?;
        println!("Create link {} -> {}", source.display(), target.display());
        Ok(std::os::unix::fs::symlink(&source, &target)?)
    }

    pub(crate) fn unlink(&self) -> Result<()> {
        if let Ok(target) = &self.target_path() {
            if let Ok(metadata) = fs::symlink_metadata(target) {
                let file_type = metadata.file_type();
                let is_symlink = file_type.is_symlink();

                if target.exists() && is_symlink {
                    if target.is_dir() {
                        println!("Removing link {}", target.to_str().unwrap());
                        let _ = fs::remove_dir_all(&target);
                    } else if target.is_file() {
                        println!("Removing link {}", target.to_str().unwrap());
                        let _ = fs::remove_file(&target);
                    }
                }
            }
        };

        Ok(())
    }
}

impl Settings {
    /// Try to find a theme matching `[theme.name] config property`
    /// Fallback to the default `Theme` impl is none is found
    pub fn load_theme(&self) -> Theme {
        let theme_settings = &self.theme.as_ref();

        match theme_settings {
            Some(theme_settings) => {
                match theme_settings.name.as_ref() {
                    Some(theme_name) => match self.resolve_theme(theme_name) {
                        Ok(theme) => theme,
                        Err(err) => {
                            eprintln!("{}. Falling back to default theme", err);
                            Theme::default()
                        }
                    },
                    None => {
                        eprintln!("[theme.name] is not set in bombadil.toml falling back to default theme");
                        Theme::default()
                    }
                }
            }
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
                    Err(anyhow!(
                        "Unable to find bombadil config file {}",
                        path.display()
                    ))
                }
            }
            Err(err) => Err(anyhow!("Config error : {}", err)),
        }
    }

    fn resolve_theme(&self, theme_name: &str) -> Result<Theme> {
        let theme_path = self
            .theme_dir()
            .map(|theme_dir| theme_dir.join(theme_name))
            .map_err(|_| {
                anyhow!(
                    "Theme not found {:?}, falling back to default theme",
                    theme_name
                )
            })?;

        // unwrapping `theme_path` is safe here
        let content = fs::read_to_string(&theme_path).map_err(|err| {
            anyhow!(
                "Cannot read {}, cause : {}",
                theme_path.to_str().unwrap(),
                err
            )
        })?;

        toml::from_str(&content).map_err(|err| anyhow!("Cannot parse theme : {}", err))
    }

    /// Resolve `$XDG_CONFIG_DIR`
    pub fn xdg_config_dir() -> Result<PathBuf, ConfigError> {
        dirs::config_dir().ok_or(ConfigError::NotFound(
            "Unable to find XDG_CONFIG_DIR".into(),
        ))
    }

    /// Resolve the bombadil XDG settings path : `$XDG_CONFIG_DIR/bombadil.toml
    pub fn bombadil_config_xdg_path() -> Result<PathBuf, ConfigError> {
        dirs::config_dir()
            .ok_or(ConfigError::NotFound(
                "Unable to find `$XDG_CONFIG/bombadil.toml`".into(),
            ))
            .map(|path| path.join("bombadil.toml"))
    }

    /// Resolve the bombadil XDG settings path : `$HOME/{dotfiles}/bombadil.toml`
    pub fn bombadil_config_dot_path(&self) -> Result<PathBuf, ConfigError> {
        Ok(self.bombadil_dotfile_path()?.join("bombadil.toml"))
    }

    /// Resolve the bombadil XDG theme path : `$HOME/{dotfiles}/bombadil.toml`
    pub fn theme_dir(&self) -> Result<PathBuf, ConfigError> {
        Ok(self.bombadil_dotfile_path()?.join("themes"))
    }

    /// Resole the bombadil user defined path
    pub fn bombadil_dotfile_path(&self) -> Result<PathBuf, ConfigError> {
        dirs::home_dir()
            .ok_or(ConfigError::NotFound(
                "Unable to find $HOME directory".into(),
            ))
            .map(|home| home.join(&self.dotfiles_dir))
            .map_err(|_err| {
                ConfigError::NotFound("Unable to find toml setting `dotfile_dir`".into())
            })
    }

    /// Resole the bombadil path theme path in dotfiles user dir
    pub fn bombadil_dots_user_theme_path(&self) -> Result<PathBuf, ConfigError> {
        dirs::home_dir()
            .ok_or(ConfigError::NotFound(
                "Unable to find $HOME directory".into(),
            ))
            .map(|home| home.join(&self.dotfiles_dir).join("themes"))
            .map_err(|_err| ConfigError::NotFound("Unable to find themes directory".into()))
    }
}
