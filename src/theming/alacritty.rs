use crate::theming::Theme;

#[derive(Debug, Serialize, Clone)]
pub(crate) struct AlacrityColors {
    pub primary: PrimaryColors,
    pub cursor: CursorColors,
    pub normal: Palette,
    pub bright: Palette,
}

#[derive(Debug, Serialize, Clone)]
pub(crate) struct PrimaryColors {
    pub background: String,
    pub foreground: String,
}

#[derive(Debug, Serialize, Clone)]
pub(crate) struct CursorColors {
    pub text: String,
    pub cursor: String,
}

#[derive(Debug, Serialize, Clone)]
pub(crate) struct Palette {
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
}

impl AlacrityColors {
    pub(crate) fn from_theme(theme: Theme) -> Self {
        AlacrityColors {
            primary: PrimaryColors {
                background: theme.background,
                foreground: theme.foreground,
            },
            cursor: CursorColors {
                text: theme.text,
                cursor: theme.cursor,
            },
            normal: Palette {
                black: theme.black,
                red: theme.red,
                green: theme.green,
                yellow: theme.yellow,
                blue: theme.blue,
                magenta: theme.magenta,
                cyan: theme.cyan,
                white: theme.white,
            },
            bright: Palette {
                black: theme.light_black,
                red: theme.light_red,
                green: theme.light_green,
                yellow: theme.light_yellow,
                blue: theme.light_blue,
                magenta: theme.light_magenta,
                cyan: theme.light_cyan,
                white: theme.light_white,
            },
        }
    }
}

#[cfg(test)]
mod test {

    //
    // #[test]
    // fn de_ok() {
    //     AlacrityColors::write().unwrap();
    // }
}
