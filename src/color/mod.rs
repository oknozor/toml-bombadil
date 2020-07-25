mod sway_color;
mod alacritty_colors;
mod wofi_colors;

pub struct Theme {

}

pub trait  ToConfig {
    fn to_config(&self) -> String;
}