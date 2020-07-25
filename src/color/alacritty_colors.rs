use crate::color::ToConfig;

struct AlacrityColors {
  primary_background: String,
  primary_foreground: String,
  black: String,
  red: String,
  green: String,
  yellow: String,
  blue: String,
  magenta: String,
  cyan: String,
  white: String,
  bright_black: String,
  bright_red: String,
  bright_green: String,
  bright_yellow: String,
  bright_blue: String,
  bright_magenta: String,
  bright_cyan: String,
  bright_white: String,
  text: String,
  cursor: String,
  indexed_colors: Vec<String>,
}

impl ToConfig for AlacrityColors {
  fn to_config(&self) -> String {
    todo!()
  }
}
