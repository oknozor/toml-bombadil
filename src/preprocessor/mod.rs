pub(crate) mod alacritty;
pub(crate) mod sway;
pub(crate) mod wofi;

use crate::config::SETTINGS;
use crate::theming::alacritty::AlacrityColors;
use crate::theming::sway::SwayColor;
use crate::theming::wofi::Wofi;
use crate::theming::{Theme, ThemeLocation};
use anyhow::Result;
use std::marker::PhantomData;

pub struct AlacrittyPreprocessor {
    location: ThemeLocation<AlacrityColors>,
}

pub struct WofiPreprocessor {
    location: ThemeLocation<Wofi>,
}

pub struct SwayPreprocessor {
    location: ThemeLocation<SwayColor>,
}

impl Preprocessor<AlacrityColors> for AlacrittyPreprocessor {
    fn get() -> Option<Box<Self>> {
        SETTINGS
            .theme
            .as_ref()
            .map(|theme| theme.alacritty.as_ref())
            .and_then(|location| {
                location.map(|location| AlacrittyPreprocessor {
                    location: location.clone(),
                })
            })
            .map(|preprocessor| Box::new(preprocessor))
    }

    fn location(&self) -> &ThemeLocation<AlacrityColors> {
        &self.location
    }
}

impl Preprocessor<SwayColor> for SwayPreprocessor {
    fn get() -> Option<Box<Self>> {
        SETTINGS
            .theme
            .as_ref()
            .map(|theme| theme.sway.as_ref())
            .and_then(|location| {
                location.map(|location| SwayPreprocessor {
                    location: location.clone(),
                })
            })
            .map(|preprocessor| Box::new(preprocessor))
    }

    fn location(&self) -> &ThemeLocation<SwayColor> {
        &self.location
    }
}

impl Preprocessor<Wofi> for WofiPreprocessor {
    fn get() -> Option<Box<Self>> {
        SETTINGS
            .theme
            .as_ref()
            .map(|theme| theme.wofi.as_ref())
            .and_then(|location| {
                location.map(|location| WofiPreprocessor {
                    location: location.clone(),
                })
            })
            .map(|preprocessor| Box::new(preprocessor))
    }

    fn location(&self) -> &ThemeLocation<Wofi> {
        &self.location
    }
}

pub trait Preprocessor<T>
where
    T: Theming,
{
    fn get() -> Option<Box<Self>>;

    fn location(&self) -> &ThemeLocation<T>;

    fn execute(&self) -> Result<()> {
        let content = self.location().get_content()?;
        let content = T::apply_theme(SETTINGS.get_current_theme_or_default(), &content)?;

        let path = self.location().get_path()?;
        std::fs::write(&path, content)
            .map_err(|err| anyhow!("Failed to write theme to {:?}: {}", path, err))
    }
}

pub trait Theming {
    fn apply_theme(theme: Theme, content: &str) -> Result<String>;
    // Trick to get phantom type from `Self` when deserializing ThemeLocation<Self>
    fn get_type() -> PhantomData<Self> {
        PhantomData::<Self>
    }
}
