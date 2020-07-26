use crate::color::alacritty_colors::AlacrityColors;
use std::path::{Path, PathBuf};
use anyhow::Result;
use crate::config::Settings;

pub(crate) mod sway_color;
pub(crate) mod alacritty_colors;
pub(crate) mod wofi_colors;

pub trait ToConfig {
    fn write() -> Result<()>;
    fn from_theme(theme: Theme) -> Self;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ThemeConfig {
    pub(crate) name: Option<String>,
    alacritty: Option<ThemeLocation>,
    sway: Option<ThemeLocation>,
    wofi: Option<ThemeLocation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ThemeLocation {
    source: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Theme {
    background: String,
    foreground: String,
    text: String,
    cursor: String,
    black: String,
    red: String,
    green: String,
    yellow: String,
    blue: String,
    magenta: String,
    cyan: String,
    white: String,
    light_black: String,
    light_red: String,
    light_green: String,
    light_yellow: String,
    light_blue: String,
    light_magenta: String,
    light_cyan: String,
    light_white: String,
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
            light_white: "#ECEFF4".to_string()
        }
    }
}

impl ThemeLocation {
    fn get_path(&self) -> Result<PathBuf> {
        let mut xdg_config_path = Settings::xdg_config_dir()?;
        xdg_config_path.push(&self.source);
        Ok(xdg_config_path)
    }
}