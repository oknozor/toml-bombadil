#[macro_use]
extern crate indoc;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate anyhow;


use crate::config::Settings;
use anyhow::Result;
use std::path::{PathBuf, Path};
use std::ops::Not;
use std::os::unix::fs;

mod color;
mod config;

pub fn edit_links() -> Result<()> {
    let settings = Settings::get()?;
    println!("{:?}", settings);
    let home = dirs::home_dir();

    if let Some(home) = home {
        // append dotfiles path to $HOME
        let mut dot_dir = home.clone();
        dot_dir.push(&settings.dotfiles_dir);

        if dot_dir.exists() {
            settings.dot.iter()
                .for_each(|dot| {
                    // append source to dotfiles path
                    let mut source = PathBuf::from(&dot_dir);
                    source.push(&dot.source);


                    // append relative target to $HOME
                    let target = PathBuf::from(&dot.target);
                    if target.is_absolute() {
                        link(&source, &target).unwrap();
                    } else {
                        let mut relative_target = PathBuf::from(&home);
                        relative_target.push(&dot.target);
                        link(&source, &relative_target).unwrap();
                    }
                });
        }
    }

    Ok(())
}

pub fn self_link(dot_config_path: &str) -> Result<()> {
    let bombadil_xdg_config = Settings::xdg_path()?;
    let bombadil_config_local = Path::new(dot_config_path).to_path_buf();
    if bombadil_config_local.exists() {
        link(&bombadil_config_local, &bombadil_xdg_config)
    } else {
        Err(anyhow!("Config file {:?} not found", bombadil_config_local))
    }
}

fn link(source: &PathBuf, target: &PathBuf) -> Result<()> {
    let source = source.canonicalize()?;

    if source.exists().not() {
       return Err(anyhow!("source dir {:?} does not exists", source))
    }

    if let Some(target_dir) = target.parent() {
        target_dir.canonicalize()
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
