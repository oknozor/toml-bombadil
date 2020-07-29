#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate pest_derive;

use crate::config::{Settings, SETTINGS};
use crate::preprocessor::Preprocessor;
use crate::preprocessor::{AlacrittyPreprocessor, SwayPreprocessor, WofiPreprocessor};
use crate::theming::{ARGONAUT, AYU};
use anyhow::Result;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::ops::Not;
use std::os::unix::fs;
use std::path::{Path, PathBuf};

pub(crate) mod config;
pub(crate) mod preprocessor;
pub mod theming;

pub(crate) trait AsConfigPath {
    fn path() -> Path;
}

/// Create a symlink for each [[dot]] config defined in bombadil.toml
pub fn edit_links() -> Result<()> {
    // FIXME : unwrap usage
    SETTINGS.dot.iter().for_each(|dot| {
        let _ = dot.unlink().expect(&format!("UNLINK ERROR {:?}", &dot));
        dot.link().expect(&format!("DOT ERROR {:?}", &dot))
    });
    Ok(())
}

/// 1. Create a symlink in XDG_CONFIG_DIR/bombadil.toml pointing to {dotfiles}/bombadil.toml
/// to allow usage from anywhere in the filesystem.
/// 2. Create bombadil default directories
///   - theme : $HOME/{dotfiles}/themes
///   - default themes: $HOME/{dotfiles}/themes/{theme}.toml
pub fn install(dotfiles_path: &str) -> Result<()> {
    // Link bombadil config against xdg dir
    let dotfiles_path = Path::new(dotfiles_path);

    if !dotfiles_path.exists() {
        return Err(anyhow!("{}, not found"));
    }

    self_link(dotfiles_path)?;

    // Create default themes
    let theme_dir = dotfiles_path.join("themes");
    if !theme_dir.exists() {
        std::fs::create_dir(theme_dir)?
    }

    write_theme(AYU)?;
    write_theme(ARGONAUT)
}

/// Execute preprocessors defined in bombadil.toml to change the current theme.
pub fn load_theme() -> Result<()> {
    if let Some(_) = &SETTINGS.theme {
        if let Some(processor) = AlacrittyPreprocessor::get() {
            processor.execute()?
        }

        if let Some(processor) = SwayPreprocessor::get() {
            processor.execute()?
        }

        if let Some(processor) = WofiPreprocessor::get() {
            processor.execute()?
        }
    } else {
        eprintln!("No theme entry in bombadil config")
    }
    Ok(())
}

pub fn list_themes() -> Result<Vec<PathBuf>> {
    let theme_path = SETTINGS.theme_dir()?;
    std::fs::read_dir(theme_path)
        .map(|read_dir| {
            read_dir
                .map(|item| item.unwrap().path())
                .collect()
        })
        .map_err(|err| anyhow!("Cannot read config directory, {}", err))
}

fn self_link<T>(dot_config_path: T) -> Result<()>
where
    T: AsRef<OsStr>,
{
    let bombadil_xdg_config = Settings::bombadil_config_xdg_path()?;

    let bombadil_config_local = Path::new(&dot_config_path).join("bombadil.toml");

    if bombadil_config_local.exists() {
        link(&bombadil_config_local, &bombadil_xdg_config)
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


fn write_theme((theme_name, theme): (&str, &[u8])) -> Result<()> {
    let path = SETTINGS.theme_dir().unwrap().join(theme_name);

    if !path.exists() {
        File::create(&path)
            .map_err(|err| anyhow!("Error with theme {:?}. {}", path, err))?
            .write_all(theme)
            .map_err(|err| anyhow!("Could not write to file {:?} : {}", path, err))?;
    }

    Ok(())
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
