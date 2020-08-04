#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate pest_derive;

use crate::dots::DotLink;
use crate::settings::Settings;
use crate::templating::Variables;
use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::ops::Not;
use std::os::unix::fs;
use std::path::{PathBuf, Path};
use dirs::home_dir;

pub(crate) mod dots;
pub(crate) mod hook;
pub(crate) mod settings;
pub(crate) mod templating;

pub struct Bombadil {
    path: PathBuf,
    dots: Vec<DotLink>,
    vars: Variables,
    hooks: Vec<String>,
}

impl Bombadil {
    pub fn link_self_config(&self, config_path: Option<PathBuf>) -> Result<()> {
        let xdg_config_dir = dirs::config_dir();
        if xdg_config_dir.is_none() {
            return Err(anyhow!("$XDG_CONFIG does not exists",));
        }

        let xdg_config = Settings::bombadil_config_xdg_path()?;

        if let Ok(_) = std::fs::symlink_metadata(&xdg_config) {
            std::fs::remove_file(&xdg_config)?;
        }

        let config_path = &config_path
            .unwrap_or(Path::new("bombadil.toml").to_path_buf())
            .canonicalize()?;

        let config_path = if config_path.is_dir() {
            config_path.join("bombadil.toml")
        } else {
            config_path.to_owned()
        };

        fs::symlink(&config_path, &xdg_config)
            .map_err(|err| anyhow!("Unable to symlink {:?} to {:?} : {}",config_path, xdg_config, err))
    }

    pub fn install(&self) -> Result<()> {
        if !self.path.exists() {
            return Err(anyhow!("Dotfiles base path : {}, not found"));
        }

        let dot_copy_dir = &self.path.join(".dots");

        if dot_copy_dir.exists() {
            std::fs::remove_dir_all(&dot_copy_dir)?;
        }

        std::fs::create_dir(dot_copy_dir)?;

        for dot in self.dots.iter() {
            let dot_source_path = self.source_path(&dot.source)?;
            let dot_copy_path = self.dot_copy_source_path(&dot.source);

            self.traverse_dots_and_copy(&dot_source_path, &dot_copy_path)?;

            let target = &dot.target()?;

            // Unlink if exists
            if let Ok(_) = std::fs::symlink_metadata(target) {
                if target.is_dir() {
                    std::fs::remove_dir_all(&target)?;
                } else {
                    std::fs::remove_file(&target)?;
                }
            }

            fs::symlink(&dot_copy_path, target)?;
        }
        // TODO : EXECUTE HOOK
        Ok(())
    }

    pub fn from_settings() -> Result<Bombadil> {
        let settings = Settings::get()?;
        let base_dir = home_dir().unwrap()
            .join(&settings.dotfiles_dir);
        // Resolve variables from path
        let mut vars = Variables::default();
        if let Some(setting_vars) = settings.var {
            for var in setting_vars {
                let template = Variables::from_toml(&base_dir.join(var.path))?;
                vars.extend(template);
            }
        }

        // Resolve hooks from config
        let mut hooks = vec![];
        if let Some(setting_hooks) = settings.hook {
            hooks.extend(setting_hooks
                .iter()
                .map(|hook| &hook.command)
                .cloned()
                .collect::<Vec<String>>());
        }

        let home_dir = dirs::home_dir();
        if home_dir.is_none() {
            return Err(anyhow!("$HOME directory not found"));
        }

        let path = home_dir
            .expect("Unexpected error")
            .join(settings.dotfiles_dir)
            .canonicalize()?;

        if path.exists().not() {
            return Err(anyhow!("Config file {:?} does not exists", &path));
        }

        Ok(Self {
            path,
            dots: settings.dot,
            vars,
            hooks,
        })
    }

    fn traverse_dots_and_copy(&self, source_path: &PathBuf, copy_path: &PathBuf) -> Result<()> {
        // Single file : inject vars and write to .dots/
        if source_path.is_file() {
            let content = self.vars.to_dot(&source_path)?;
            std::fs::create_dir_all(&copy_path.parent().unwrap())?;
            let mut dot_copy = File::create(&copy_path)?;
            dot_copy.write_all(content.as_bytes())?;
        } else if source_path.is_dir() {
            std::fs::create_dir_all(copy_path)?;
            for entry in source_path.read_dir()? {
                let entry_name = entry?.path();
                let entry_name = entry_name.file_name().unwrap().to_str().unwrap();
                &self.traverse_dots_and_copy(&source_path.join(entry_name), &copy_path.join(entry_name));
            }
        }

        Ok(())
    }

    /// Resolve dot source copy path ({dotfiles/.dots/) against user defined dotfile directory
    /// Check if file exists
    fn source_path(&self, dot_source_path: &PathBuf) -> Result<PathBuf> {
        let path = self.path.join(&dot_source_path);

        if path.exists() {
            Ok(path)
        } else {
            Err(anyhow!("Path does not exists: {:?}", path))
        }
    }

    /// Resolve dot source copy path ({dotfiles/.dots/) against user defined dotfile directory
    /// Does not check if file exists
    pub(crate) fn dot_copy_source_path(&self, source: &PathBuf) -> PathBuf {
        self.path
            .join(".dots")
            .join(source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use temp_testdir::TempDir;
    use std::collections::HashMap;

    #[test]
    fn self_link_works() {

        // Arrange
        let config = Bombadil {
            path: Path::new("tests/dotfiles_simple").to_path_buf().canonicalize().unwrap(),
            dots: vec![],
            vars: Variables::default(),
            hooks: vec![],
        };

        // Act
        let config_path = Path::new("tests/dotfiles_simple/bombadil.toml").to_path_buf();
        config.link_self_config(Some(config_path)).unwrap();
        let link = dirs::config_dir().unwrap().join("bombadil.toml");

        // Assert
        assert!(link.exists());
    }

    #[test]
    fn install_single_file_works() {

        // Arrange
        let temp_dir = TempDir::new("test_tmp", false).to_path_buf();
        let target = &temp_dir.join("dot_target");

        let mut map = HashMap::new();
        map.insert("red".to_string(), "red_value".to_string());

        let config = Bombadil {
            path: Path::new("tests/dotfiles_simple").to_path_buf().canonicalize().unwrap(),
            dots: vec![DotLink {
                source: Path::new("template").to_path_buf(),
                target: target.clone(),
            }],
            vars: Variables {
                variables: map
            },
            hooks: vec![],
        };

        // Act
        config.install().unwrap();

        // Assert
        assert!(target.exists());
        assert_eq!(std::fs::read_to_string(&target).unwrap(), "color: red_value".to_string());
    }

    #[test]
    fn install_with_subdir() {

        // Arrange
        let temp_dir = TempDir::new("test_tmp", false).to_path_buf();
        let target = &temp_dir.join("sub_dir_target");

        let mut map = HashMap::new();
        map.insert("red".to_string(), "red_value".to_string());
        map.insert("blue".to_string(), "blue_value".to_string());

        let config = Bombadil {
            path: Path::new("tests/dotfiles_nested").to_path_buf().canonicalize().unwrap(),
            dots: vec![DotLink {
                source: Path::new("sub_dir").to_path_buf(),
                target: target.clone(),
            }],
            vars: Variables {
                variables: map
            },
            hooks: vec![],
        };

        // Act
        config.install().unwrap();

        // Assert
        assert!(target.exists());
        let path = &target.read_link().unwrap();
        let red_dot = std::fs::read_to_string(path.join("template_1")).unwrap();
        let blue_dot = std::fs::read_to_string(path.join("template_2")).unwrap();
        assert_eq!(red_dot, "color: red_value".to_string());
        assert_eq!(blue_dot, "color: blue_value".to_string());
    }

    #[test]
    fn install_with_nested_subdirs() {

        // Arrange
        let temp_dir = TempDir::new("test_tmp", false).to_path_buf();
        let target = &temp_dir.join("sub_dir_2_target");

        let mut map = HashMap::new();
        map.insert("red".to_string(), "red_value".to_string());
        map.insert("blue".to_string(), "blue_value".to_string());

        let config = Bombadil {
            path: Path::new("tests/dotfiles_nested_2").to_path_buf().canonicalize().unwrap(),
            dots: vec![DotLink {
                source: Path::new("sub_dir_1").to_path_buf(),
                target: target.clone(),
            }],
            vars: Variables {
                variables: map
            },
            hooks: vec![],
        };

        // Act
        config.install().unwrap();

        // Assert
        assert!(target.exists());
        let path = &target.read_link().unwrap();
        let red_dot = std::fs::read_to_string(path.join("template_1")).unwrap();
        let blue_dot = std::fs::read_to_string(path.join("subdir_2").join("template_2")).unwrap();
        assert_eq!(red_dot, "color: red_value".to_string());
        assert_eq!(blue_dot, "color: blue_value".to_string());
    }
}
