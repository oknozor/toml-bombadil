use crate::config::Settings;
use crate::preprocessor::Theming;
use crate::theming::alacritty::AlacrityColors;
use crate::theming::sway::SwayColor;
use crate::theming::wofi::Wofi;
use anyhow::Result;
use std::fs;
use std::marker::PhantomData;
use std::path::{PathBuf, Path};

pub(crate) mod alacritty;
pub(crate) mod sway;
pub(crate) mod wofi;
pub(crate) mod display;

pub static ARGONAUT: (&str, &[u8]) = (
    "argonaut.toml",
    include_bytes!("default_themes/argonaut.toml"),
);

pub static AYU: (&str, &[u8]) = (
    "ayu_mirage.toml",
    include_bytes!("default_themes/ayu_mirage.toml"),
);

#[derive(Debug, Deserialize, Serialize)]
pub struct ThemeConfig {
    pub(crate) name: Option<String>,
    pub(crate) alacritty: Option<ThemeLocation<AlacrityColors>>,
    pub(crate) sway: Option<ThemeLocation<SwayColor>>,
    pub(crate) wofi: Option<ThemeLocation<Wofi>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ThemeLocation<T: Theming> {
    source: String,
    #[serde(default = "T::get_type")]
    phantom: PhantomData<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Theme {
    pub background: String,
    pub foreground: String,
    pub text: String,
    pub cursor: String,
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
    pub light_black: String,
    pub light_red: String,
    pub light_green: String,
    pub light_yellow: String,
    pub light_blue: String,
    pub light_magenta: String,
    pub light_cyan: String,
    pub light_white: String,
}

impl Theme {
    pub fn from_path<T>(theme_path: T) -> Result<Theme> where T: AsRef<Path> {
        // unwrapping `theme_path` is safe here
        let content = fs::read_to_string(&theme_path).map_err(|err| {
            anyhow!(
                "Cannot read {}, cause : {}",
                theme_path.as_ref().display(),
                err
            )
        })?;

        toml::from_str(&content).map_err(|err| anyhow!("Cannot preprocessor theme : {}", err))
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            background: "#3b4252".to_string(),
            foreground: "#D8DEE9".to_string(),
            text: "#2E3440".to_string(),
            cursor: "#D8DEE9".to_string(),
            black: "#3B4252".to_string(),
            red: "#BF616A".to_string(),
            green: "#A3BE8C".to_string(),
            yellow: "#EBCB8B".to_string(),
            blue: "#81A1C1".to_string(),
            magenta: "#B48EAD".to_string(),
            cyan: "#88C0D0".to_string(),
            white: "#E5E9F0".to_string(),
            light_black: "#4C566A".to_string(),
            light_red: "#BF616A".to_string(),
            light_green: "#A3BE8C".to_string(),
            light_yellow: "#EBCB8B".to_string(),
            light_blue: "#81A1C1".to_string(),
            light_magenta: "#B48EAD".to_string(),
            light_cyan: "#8FBCBB".to_string(),
            light_white: "#ECEFF4".to_string(),
        }
    }
}

impl<T: Theming> ThemeLocation<T> {
    /// Return the `source` path for a theme configuration
    pub fn get_path(&self) -> Result<PathBuf> {
        let mut xdg_config_path = Settings::xdg_config_dir()?;
        xdg_config_path.push(&self.source);
        Ok(xdg_config_path)
    }

    /// Return the content of a specific program configuration theme (ex: $HOME/.config/allacrity/allacrity.yaml)
    pub fn get_content(&self) -> Result<String> {
        let path = &self.get_path()?;
        fs::read_to_string(path)
            .map_err(|err| anyhow!("cannot read theme location {:?} : {}", path, err))
    }
}
