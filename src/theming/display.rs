use crate::theming::Theme;
use termion::color;
use std::fmt;
use serde::export::Formatter;
use termion::color::{Rgb, Bg};
use anyhow::Result;
use std::cmp::Ordering;

struct RgbWrapper {
    rgb: Rgb
}

impl RgbWrapper {
    fn from_hex(hex: &str) -> Result<RgbWrapper> {
        let r = u8::from_str_radix(&hex[1..3], 16)?;
        let g = u8::from_str_radix(&hex[3..5], 16)?;
        let b = u8::from_str_radix(&hex[5..7], 16)?;
        let rgb = color::Rgb(r, g, b);
        Ok(RgbWrapper { rgb })
    }

    fn to_hue(&self) -> i64 {
        let rgb = self.rgb;
        let (r, g, b) = (rgb.0 as f64, rgb.1 as f64, rgb.2 as f64);

        let sqrt3 = (3f64).sqrt();
        let hue = (sqrt3 * (g - b)).atan2((2f64 * r) - g - b);
        let hue = hue.to_degrees() as i64;
        let modulo = hue % 360;


        if modulo == 0  ||  hue > 0 {
            hue
        } else {
            360 + hue
        }
    }

    fn print(&self) {
        let color = Bg(self.rgb);
        print!("{}  ", color);
        print!("{}  ", color);
    }
}

impl Eq for RgbWrapper {}

impl Ord for RgbWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_hue().cmp(&other.to_hue())
    }
}

impl PartialOrd for RgbWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl PartialEq for RgbWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.to_hue() == other.to_hue()
    }
}


impl fmt::Display for Theme {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        self.write_color_line();
        self.write_color_line();
        self.write_color_line();
        self.write_color_line();
        println!("{}    ", Bg(color::Reset));
        Ok(())
    }
}

impl Theme {
    fn to_rgb_vec_sorted(&self) -> Result<Vec<RgbWrapper>> {
        let mut colors = vec![
            RgbWrapper::from_hex(&self.blue)?,
            RgbWrapper::from_hex(&self.light_blue)?,
            RgbWrapper::from_hex(&self.foreground)?,
            RgbWrapper::from_hex(&self.text)?,
            RgbWrapper::from_hex(&self.cursor)?,
            RgbWrapper::from_hex(&self.cyan)?,
            RgbWrapper::from_hex(&self.light_cyan)?,
            RgbWrapper::from_hex(&self.magenta)?,
            RgbWrapper::from_hex(&self.light_magenta)?,
            RgbWrapper::from_hex(&self.black)?,
            RgbWrapper::from_hex(&self.light_black)?,
            RgbWrapper::from_hex(&self.light_white)?,
            RgbWrapper::from_hex(&self.white)?,
            RgbWrapper::from_hex(&self.red)?,
            RgbWrapper::from_hex(&self.light_red)?,
            RgbWrapper::from_hex(&self.green)?,
            RgbWrapper::from_hex(&self.light_green)?,
        ];

        colors.sort();

        Ok(colors)
    }

    fn write_color_line(&self) {
        println!();
        self.to_rgb_vec_sorted().unwrap().iter()
            .for_each(|rgb| rgb.print());
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_convert_to_rgb() {
        let result = RgbWrapper::from_hex("#48a808").unwrap();
        assert_eq!(result.rgb, Rgb(72, 168, 8));
    }

    #[test]
    fn should_convert_red_hue() {
        let red = RgbWrapper { rgb: Rgb(255, 0, 0) };

        let hue = red.to_hue();

        assert_eq!(hue, 0);
    }

    #[test]
    fn should_convert_blue_hue() {
        let blue = RgbWrapper { rgb: Rgb(0, 0, 255) };

        let hue = blue.to_hue();

        assert_eq!(hue, 240);
    }

    #[test]
    fn should_convert_green_hue() {
        let green = RgbWrapper { rgb: Rgb(0, 255, 0) };

        let hue = green.to_hue();

        assert_eq!(hue, 120);
    }
}