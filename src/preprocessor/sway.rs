use crate::preprocessor::Theming;
use crate::theming::sway::SwayColor;
use crate::theming::Theme;
use anyhow::Result;

fn sway_var(key: &str, value: String) -> String {
    format!("set ${}  {}", key, value)
}

impl Theming for SwayColor {
    fn apply_theme(theme: Theme, _: &str) -> Result<String> {
        let mut colors = vec![];
        colors.push(sway_var("text", theme.text));
        colors.push(sway_var("cursor", theme.cursor));
        colors.push(sway_var("background", theme.background));
        colors.push(sway_var("foreground", theme.foreground));
        colors.push(sway_var("black", theme.black));
        colors.push(sway_var("white", theme.white));
        colors.push(sway_var("red", theme.red));
        colors.push(sway_var("blue", theme.blue));
        colors.push(sway_var("green", theme.green));
        colors.push(sway_var("cyan", theme.cyan));
        colors.push(sway_var("magenta", theme.magenta));
        colors.push(sway_var("yellow", theme.yellow));
        colors.push(sway_var("light_black", theme.light_black));
        colors.push(sway_var("light_white", theme.light_white));
        colors.push(sway_var("light_red", theme.light_red));
        colors.push(sway_var("light_blue", theme.light_blue));
        colors.push(sway_var("light_green", theme.light_green));
        colors.push(sway_var("light_cyan", theme.light_cyan));
        colors.push(sway_var("light_magenta", theme.light_magenta));
        colors.push(sway_var("light_yellow", theme.light_yellow));

        Ok(SwayColor { colors }.to_string())
    }
}
