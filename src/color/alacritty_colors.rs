use crate::color::{ToConfig, Theme};
use std::collections::BTreeMap;
use crate::config::Settings;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read};
use anyhow::Result;
use std::fs;
use serde_yaml::{Value, Index, Mapping, Sequence};

#[derive(Debug, Serialize)]
pub(crate) struct AlacrityColors {
    primary: PrimaryColors,
    cursor: CursorColors,
    normal: Palette,
    bright: Palette
}

#[derive(Debug, Serialize)]
pub struct PrimaryColors {
    background: String,
    foreground: String,
}

#[derive(Debug, Serialize)]
pub struct CursorColors {
    text: String,
    cursor: String,
}

#[derive(Debug, Serialize)]
pub struct Palette {
    black: String,
    red: String,
    green: String,
    yellow: String,
    blue: String,
    magenta: String,
    cyan: String,
    white: String,
}

impl ToConfig for AlacrityColors {
    fn write() -> Result<()> {
        // Get alacritty config from bombadil config
        let settings = &Settings::get().unwrap();
        let allacrity_config_path = &settings.theme.as_ref().unwrap().alacritty;
        let allacrity_config_path = allacrity_config_path.as_ref().unwrap().get_path()?;
        let content = fs::read_to_string(&allacrity_config_path)?;
        let mut yaml = serde_yaml::from_str::<Value>(&content)?;
        let colors = yaml.get_mut("colors").unwrap();

        // Create a new alacritty theme from bombadil color scheme
        let new_theme = AlacrityColors::from_theme(settings.load_theme());

        // Replace and write alacritty config
        *colors = serde_yaml::to_value(new_theme)?;
        std::fs::write(allacrity_config_path, serde_yaml::to_string(&yaml)?)?;
        Ok(())
    }

    fn from_theme(theme: Theme) -> Self {
        AlacrityColors {
            primary: PrimaryColors { background: theme.background, foreground: theme.foreground },
            cursor: CursorColors { text: theme.text, cursor: theme.cursor },
            normal: Palette {
                black: theme.black,
                red: theme.red,
                green: theme.green,
                yellow: theme.yellow,
                blue: theme.blue,
                magenta: theme.magenta,
                cyan: theme.cyan,
                white: theme.white
            },
            bright: Palette {
                black: theme.light_black,
                red: theme.light_red,
                green: theme.light_green,
                yellow: theme.light_yellow,
                blue: theme.light_blue,
                magenta: theme.light_magenta,
                cyan: theme.light_cyan,
                white: theme.light_white
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn de_ok() {
        AlacrityColors::write().unwrap();
    }
}
