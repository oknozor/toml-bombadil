use crate::theming::Theme;
use termion::color;
use std::fmt;
use serde::export::Formatter;
use termion::color::Rgb;
use anyhow::Result;

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let blue = color::Bg(hex_to_rgb(&self.blue).unwrap());
        let light_blue = color::Bg(hex_to_rgb(&self.light_blue).unwrap());
        let cyan = color::Bg(hex_to_rgb(&self.cyan).unwrap());
        let light_cyan = color::Bg(hex_to_rgb(&self.light_cyan).unwrap());
        write!(f, "({}blue, {}light_blue, {}cyan, {}light_cyan)", blue, light_blue, cyan, light_cyan)
    }
}


fn hex_to_rgb(hex: &str) -> Result<Rgb> {
    let r = u8::from_str_radix(&hex[1..3], 16)?;
    let g = u8::from_str_radix(&hex[3..5], 16)?;
    let b = u8::from_str_radix(&hex[5..7], 16)?;

    Ok(color::Rgb(r, g, b))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_convert_to_rgb() {

        let result = hex_to_rgb("#48a808").unwrap();
        assert_eq!(result, Rgb(72, 168, 8));
    }
}