use crate::preprocessor::Theming;
use crate::theming::alacritty::AlacrityColors;
use crate::theming::Theme;
use anyhow::Result;
use serde_yaml::Value;

impl Theming for AlacrityColors {
    fn apply_theme(theme: Theme, content: &str) -> Result<String> {
        let mut yaml = serde_yaml::from_str::<Value>(&content)?;
        let colors = yaml.get_mut("colors").unwrap();
        let new_theme = AlacrityColors::from_theme(theme);
        *colors = serde_yaml::to_value(new_theme)?;

        serde_yaml::to_string(&yaml).map_err(|err| anyhow!("{}", err))
    }
}
