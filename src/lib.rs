#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate pest_derive;

use crate::theming::alacritty_theme::AlacrityColors;
use crate::theming::sway_theme::SwayColor;
use crate::theming::wofi_theme::WofiColor;
use crate::theming::{ToConfig, ARGONAUT};
use crate::config::Settings;
use anyhow::Result;
use std::ops::Not;
use std::os::unix::fs;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Write;

pub mod config;
mod theming;
mod parse;

pub fn edit_links() -> Result<()> {
    let settings = Settings::get()?;
    // FIXME : unwrap usage
    settings.dot.iter().for_each(|dot| dot.link().unwrap());
    Ok(())
}


pub fn install(dotfiles_path: &str) -> Result<()> {
    // Link bombadil config against xdg dir
    let dotfiles_path = Path::new(dotfiles_path);
    if !dotfiles_path.exists() {
        return Err(anyhow!("{}, not found"));
    }

    self_link(dotfiles_path.to_str().unwrap())?;

    // Create default themes
    let theme_path = dotfiles_path.join(".themes");
    if !theme_path.exists() {
        let theme_path = Settings::get()?.bombadil_dots_user_theme_path()?;
        std::fs::create_dir(&theme_path)
            .map_err(|err| anyhow!("Theme path {:?} already present, please run with `--force` to override. {}", &theme_path, err))?;
    }

    let argonaut = theme_path.join("aronaut.toml");

    if !argonaut.exists() {
        let mut file = File::create(theme_path.join("argonaut.toml"))?;
        file.write_all(ARGONAUT)
            .map_err(|err| anyhow!("Could not write to file {} : {}", "argonaut.toml", err))
    } else {
        Err(anyhow!("Theme argonaut is present"))
    }
}

pub fn load_theme() -> Result<()> {
    let settings = Settings::get()?;
    if let Some(theme_config) = settings.theme {
        theme_config
            .wofi
            .and_then(|_config| WofiColor::write().ok());
        theme_config
            .alacritty
            .and_then(|_config| AlacrityColors::write().ok());
        theme_config
            .sway
            .and_then(|_config| SwayColor::write().ok());
    } else {
        eprintln!("No theme entry in bombadil config")
    }
    Ok(())
}

pub fn list_themes() -> Result<Vec<Box<PathBuf>>> {
    let theme_path = Settings::bombadil_theme_xdg_path()?;
    std::fs::read_dir(theme_path)
        .map(|read_dir| read_dir
            .map(|item| Box::new(item.unwrap().path()))
            .collect())
        .map_err(|err| anyhow!("Cannot read config directory, {}", err))
}

fn self_link(dot_config_path: &str) -> Result<()> {
    std::fs::create_dir(Settings::bombadil_xdg_path()?)?;
    let bombadil_xdg_config = Settings::bombadil_config_xdg_path()?;
    let theme_xdg_config = Settings::bombadil_theme_xdg_path()?;

    let bombadil_config_local = Path::new(dot_config_path)
        .join("bombadil.toml");
    let theme_config_local = Path::new(dot_config_path)
        .join("themes");

    if bombadil_config_local.exists() {
        link(&bombadil_config_local, &bombadil_xdg_config)?;
        link(&theme_config_local, &theme_xdg_config)
    } else {
        Err(anyhow!("Config file {:?} not found", bombadil_config_local))
    }
}

fn link(source: &PathBuf, target: &PathBuf) -> Result<()> {
    let source = source.canonicalize()?;

    if source.exists().not() {
        return Err(anyhow!("source dir {:?} does not exists", source));
    }

    if let Some(target_dir) = target.parent() {
        target_dir
            .canonicalize()
            .map(|_| fs::symlink(source, &target))
            .map_err(|err| anyhow!(err))
            .map(|_| ())
    } else {
        Err(anyhow!("target dir {:?} does not exists", target))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Not;
    use std::path::PathBuf;
    use temp_testdir::TempDir;

    #[test]
    fn it_works() {
        edit_links().unwrap();
    }

    #[test]
    fn self_link_works() {
        self_link("config.toml").unwrap();
    }

    #[test]
    fn should_symlink_existing_source_file() {
// Arrange
        let temp = TempDir::default();
        let temp = PathBuf::from(temp.as_ref());

        let mut target = temp.clone();
        target.push("Ograc.lmot");
        let target = target.clone();

        let source = Path::new("Cargo.toml").to_path_buf();

// Act
        let result = link(&source, &target);

// Assert
        assert!(result.is_ok());
        assert!(Path::new(&target).exists())
    }

    #[test]
    fn should_not_symlink_invalid_source_file() {
// Arrange
        let temp = TempDir::default();
        let temp = PathBuf::from(temp.as_ref());

        let mut target = temp.clone();
        target.push("Ograc.lmot");
        let target = target.clone();

        let source = Path::new("not a file").to_path_buf();

// Act
        let result = link(&source, &target);

// Assert
        assert!(result.is_err());
        assert!(Path::new(&target).exists().not())
    }
}
