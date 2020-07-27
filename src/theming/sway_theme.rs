/*
set $black    #3b4252
set $gray     #d8dee9
set $lred     #bf616a
set $blue     #5e81ac
set $lblue    #88c0d0
set $white    #eceff4
set $yellow   #ebcb8b

# Color                 border    background text    indicator  child_border
client.focused          $yellow   $blue      $gray   $yellow     $yellow
client.focused_inactive $white    $black     $gray   $gray      $gray
client.unfocused        $black    $black     $gray   $black     $blue
client.urgent           $lred     $black     $gray   $lred      $lblue
*/

/*
wm_border
$wm_text
$wm_bg
$wm_bg_secondary
 */

use crate::theming::{Theme, ToConfig};
use crate::config::Settings;
use anyhow::Result;

#[derive(Debug, Serialize)]
pub(crate) struct SwayColor {
    colors: Vec<String>,
}

impl SwayColor {
    fn content(&self) -> String {
        self.colors.join("\n")
    }
}
impl ToConfig for SwayColor {
    fn write() -> Result<()> {
        // Get sway theming config from bombadil config
        let settings = &Settings::get()?;
        let sway_colors_config_path = &settings.theme.as_ref().unwrap().sway;
        let sway_config_path = sway_colors_config_path.as_ref().unwrap().get_path()?;

        // Create a new sway theme from bombadil theming scheme
        let new_theme = SwayColor::from_theme(settings.load_theme());

        // Replace and write sway config
        std::fs::write(sway_config_path, &new_theme.content())
            .map_err(|err| anyhow!("Sway config error : {}", err))?;
        Ok(())
    }

    fn from_theme(theme: Theme) -> Self {
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

        SwayColor { colors }
    }
}

fn sway_var(key: &str, value: String) -> String {
    format!("set ${}  {}", key, value)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn de_ok() {
        SwayColor::write().unwrap();
    }
}
