use crate::color::{Theme, ToConfig};
use crate::config::Settings;
use anyhow::Result;
use std::fs;

#[derive(Debug)]
pub struct WofiColor {
    pub window: Selector,
    pub input: Selector,
    pub input_focused: Selector,
    pub inner_box: Selector,
    pub inner_box_flowchild: Selector,
    pub scroll: Selector,
    pub outer_box: Selector,
    pub text: Selector,
}

#[derive(Debug)]
pub struct CSSProp {
    pub(crate) key: String,
    pub(crate) value: String,
}

#[derive(Debug)]
pub struct Selector {
    pub(crate) props: Vec<CSSProp>,
}

/*

*/
impl ToConfig for WofiColor {
    fn write() -> Result<()> {
        let settings = &Settings::get().unwrap();
        let wofi_theme_path = &settings.theme.as_ref().unwrap().wofi;
        let wofi_theme_path = wofi_theme_path.as_ref().unwrap().get_path()?;

        let content = fs::read_to_string(&wofi_theme_path)?;
        let mut styles = WofiColor::from_css(&content);
        let theme = settings.load_theme();

        // Replace css color props
        styles.window.set_bg_color(&theme.black);

        styles.input.set_bg_color(&theme.black);
        styles.input.set_color(&theme.white);
        styles.input.set_border_color(&theme.light_white);

        styles.input_focused.set_bg_color(&theme.light_blue);
        styles.input_focused.set_color(&theme.red);
        styles.input_focused.set_border_color(&theme.red);

        styles.text.set_bg_color(&theme.green);
        styles.text.set_color(&theme.light_magenta);

        styles.inner_box.set_bg_color(&theme.magenta);
        styles.inner_box_flowchild.set_bg_color(&theme.blue);
        styles.outer_box.set_bg_color(&theme.black);
        styles.scroll.set_bg_color(&theme.yellow);

        std::fs::write(wofi_theme_path, styles.to_string())?;
        Ok(())
    }

    // This shall never be used for Wofi, here we rely on mutating the existing config file
    // this might need some refactoring
    fn from_theme(_: Theme) -> Self {
        unreachable!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn de_ok() {
        WofiColor::write().unwrap();
    }
}
