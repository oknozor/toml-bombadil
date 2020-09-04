#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate pest_derive;

use crate::dots::Dot;
use crate::hook::Hook;
use crate::settings::{Profile, Settings};
use crate::templating::Variables;
use anyhow::Result;
use colored::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::ops::Not;
use std::os::unix::fs;
use std::path::PathBuf;

pub mod dots;
pub(crate) mod hook;
pub mod settings;
pub(crate) mod templating;

pub struct Bombadil {
    path: PathBuf,
    dots: HashMap<String, Dot>,
    vars: Variables,
    hooks: Vec<Hook>,
    profiles: HashMap<String, Profile>,
}

impl Bombadil {
    pub fn link_self_config(config_path: Option<PathBuf>) -> Result<()> {
        let xdg_config_dir = dirs::config_dir();
        if xdg_config_dir.is_none() {
            return Err(anyhow!("$XDG_CONFIG does not exists"));
        }

        let xdg_config = Settings::bombadil_config_xdg_path()?;

        if std::fs::symlink_metadata(&xdg_config).is_ok() {
            std::fs::remove_file(&xdg_config)?;
        }

        let config_path = &config_path
            .unwrap_or_else(|| PathBuf::from("bombadil.toml"))
            .canonicalize()?;

        let config_path = if config_path.is_dir() {
            config_path.join("bombadil.toml")
        } else {
            config_path.to_owned()
        };

        fs::symlink(&config_path, &xdg_config)
            .map_err(|err| {
                anyhow!(
                    "Unable to symlink {:?} to {:?} : {}",
                    config_path,
                    xdg_config,
                    err
                )
            })
            .map(|_result| {
                let source = format!("{:?}", &config_path).blue();
                let dest = format!("{:?}", &xdg_config).green();
                println!("{} => {}", source, dest)
            })
    }

    pub fn install(&self) -> Result<()> {
        self.check_dotfile_dir()?;
        let dot_copy_dir = &self.path.join(".dots");

        if dot_copy_dir.exists() {
            std::fs::remove_dir_all(&dot_copy_dir)?;
        }

        std::fs::create_dir(dot_copy_dir)?;

        for dot in self.dots.iter() {
            let dot_source_path = self.source_path(&dot.1.source);

            if let Err(err) = dot_source_path {
                eprintln!("{}", err);
                continue;
            }

            let dot_copy_path = self.dot_copy_source_path(&dot.1.source);

            self.traverse_dots_and_copy(&dot_source_path?, &dot_copy_path)?;

            let target = &dot.1.target()?;

            Bombadil::unlink(&target)?;
            Bombadil::link(&dot_copy_path, &target);
        }

        self.hooks.iter().map(Hook::run).for_each(|result| {
            if let Err(err) = result {
                eprintln!("{}", err);
            }
        });

        Ok(())
    }

    pub fn enable_profiles(&mut self, profile_keys: Vec<&str>) -> Result<()> {
        let profiles: Vec<Profile> = profile_keys
            .iter()
            // unwrap here is safe cause allowed profile keys are checked by clap
            .map(|profile_key| self.profiles.get(&profile_key.to_string()).unwrap())
            .cloned()
            .collect();

        // Merge profile dots
        for profile in profiles.iter() {
            profile.dots.iter().for_each(|(key, dot_override)| {
                // Dot exist let's override
                if let Some(dot) = self.dots.get_mut(key) {
                    if let Some(source) = &dot_override.source {
                        dot.source = source.clone()
                    }

                    if let Some(target) = &dot_override.target {
                        dot.source = target.clone()
                    }

                    if let (None, None) = (&dot_override.source, &dot_override.target) {
                        let warning = format!(
                            "Skipping {}, no `source` or `target` value to override",
                            key
                        )
                        .yellow();
                        eprintln!("{}", warning);
                    }
                // Nothing to override, let's create a new dot entry
                } else if let (Some(source), Some(target)) =
                    (&dot_override.source, &dot_override.target)
                {
                    let source = source.clone();
                    let target = target.clone();
                    self.dots.insert(key.to_string(), Dot { source, target });
                } else {
                    if dot_override.source.is_none() {
                        let warning = format!("`source` field missing for {}", key).yellow();
                        eprintln!("{}", warning);
                    }

                    if dot_override.target.is_none() {
                        let warning = format!("`target` field missing for {}", key).yellow();
                        eprintln!("{}", warning);
                    }
                }
            });

            // Add profile vars
            for path in &profile.vars {
                let variables = Variables::from_toml(&path.join(path))?;
                self.vars.extend(variables);
            }

            // Add profile hooks
            let hooks = profile
                .hooks
                .iter()
                .map(|command| command.as_ref())
                .map(Hook::new)
                .collect::<Vec<Hook>>();

            self.hooks.extend(hooks);
        }

        self.install()
    }

    fn check_dotfile_dir(&self) -> Result<()> {
        if !self.path.exists() {
            return Err(anyhow!("Dotfiles base path : {}, not found"));
        }

        if !self.path.is_dir() {
            let err = format!(
                "{} {:?} {}",
                "Provided dotfiles directory".red(),
                &self.path,
                "is not a directory".red()
            );
            return Err(anyhow!(err));
        }

        Ok(())
    }

    pub fn from_settings() -> Result<Bombadil> {
        let config = Settings::get()?;

        let home_dir = dirs::home_dir();
        if home_dir.is_none() {
            return Err(anyhow!("$HOME directory not found"));
        }

        let path = if config.dotfiles_dir.is_absolute() {
            config.dotfiles_dir
        } else {
            home_dir.unwrap().join(&config.dotfiles_dir)
        };

        if path.exists().not() {
            return Err(anyhow!("Dotfiles directory {:?} does not exists", &path));
        }

        // Resolve variables from path
        let mut vars = Variables::default();
        for var_path in config.settings.vars {
            let variables = Variables::from_toml(&path.join(&var_path))?;
            vars.extend(variables);
        }

        // Collect variable references
        let entries: Vec<(String, Option<String>)> = vars
            .variables
            .iter()
            .filter(|(_, value)| value.starts_with('%'))
            .map(|(key, value)| (key, &value[1..value.len()]))
            .map(|(key, ref_key)| (key.clone(), vars.variables.get(ref_key).cloned()))
            .collect();

        // insert value in place of references
        entries.iter().for_each(|(key, opt_value)| match opt_value {
            Some(value) => {
                let _ = vars.variables.insert(key.to_string(), value.to_string());
            }
            None => {
                let warning = format!("Reference ${} not found in config", &key).yellow();
                eprintln!("{}", warning);
            }
        });

        // Resolve hooks from config
        let hooks = config
            .settings
            .hooks
            .iter()
            .map(|cmd| Hook::new(cmd))
            .collect();

        let dots = config.settings.dots;
        let profiles = config.profiles;

        Ok(Self {
            path,
            dots,
            vars,
            hooks,
            profiles,
        })
    }

    fn traverse_dots_and_copy(&self, source_path: &PathBuf, copy_path: &PathBuf) -> Result<()> {
        // Single file : inject vars and write to .dots/
        if source_path.is_file() {
            std::fs::create_dir_all(&copy_path.parent().unwrap())?;

            if let Ok(content) = self.vars.to_dot(&source_path) {
                let mut dot_copy = File::create(&copy_path)?;
                dot_copy.write_all(content.as_bytes())?;
            } else {
                // Something went wrong parsing or reading the source path,
                // We just copy the file in place
                std::fs::copy(&source_path, &copy_path)?;
            }
        } else if source_path.is_dir() {
            std::fs::create_dir_all(copy_path)?;
            for entry in source_path.read_dir()? {
                let entry_name = entry?.path();
                let entry_name = entry_name.file_name().unwrap().to_str().unwrap();
                self.traverse_dots_and_copy(
                    &source_path.join(entry_name),
                    &copy_path.join(entry_name),
                )
                .unwrap_or_else(|err| eprintln!("{}", err));
            }
        }

        Ok(())
    }

    fn link(dot_copy_path: &PathBuf, target: &PathBuf) {
        // Link
        fs::symlink(&dot_copy_path, target)
            .map(|_result| {
                let source = format!("{:?}", &dot_copy_path).blue();
                let dest = format!("{:?}", target).green();
                println!("{} => {}", source, dest)
            })
            .map_err(|err| {
                let source = format!("{:?}", &dot_copy_path).blue();
                let dest = format!("{:?}", &target).red();
                let err = format!("{}", err).red().bold();
                anyhow!("{} => {} : {}", source, dest, err)
            })
            .unwrap_or_else(|err| eprintln!("{}", err));
    }

    fn unlink(target: &PathBuf) -> Result<()> {
        if std::fs::symlink_metadata(target).is_ok() {
            if target.is_dir() {
                std::fs::remove_dir_all(&target)?;
            } else {
                std::fs::remove_file(&target)?;
            }
        }

        Ok(())
    }

    /// Resolve dot source copy path ({dotfiles/dotsource) against user defined dotfile directory
    /// Check if file exists
    fn source_path(&self, dot_source_path: &PathBuf) -> Result<PathBuf> {
        let path = self.path.join(&dot_source_path);

        if path.exists() {
            Ok(path)
        } else {
            Err(anyhow!(format!(
                "{} {:?}",
                "Path does not exists :".red(),
                path
            )))
        }
    }

    /// Resolve dot source copy path ({dotfiles/.dots/) against user defined dotfile directory
    /// Does not check if file exists
    pub(crate) fn dot_copy_source_path(&self, source: &PathBuf) -> PathBuf {
        self.path.join(".dots").join(source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::path::Path;
    use temp_testdir::TempDir;

    #[test]
    fn should_copy_dotfiles() {
        // Arrange
        let target = TempDir::new("/tmp/dot_target", false).to_path_buf();
        let mut dots = HashMap::new();
        let source = PathBuf::from("template");

        dots.insert(
            "dot".to_string(),
            Dot {
                source: source.clone(),
                target: target.clone(),
            },
        );

        let config = Bombadil {
            path: PathBuf::from("tests/dotfiles_simple")
                .canonicalize()
                .unwrap(),
            dots,
            vars: Variables::default(),
            hooks: vec![],
            profiles: Default::default(),
        };

        // Act
        config
            .traverse_dots_and_copy(
                &config.source_path(&source).unwrap(),
                &config.dot_copy_source_path(&source),
            )
            .unwrap();

        // Assert
        assert!(Path::new("tests/dotfiles_simple/.dots/template").exists());
    }

    #[test]
    fn should_copy_non_utf8_dotfiles() {
        // Arrange
        let target = TempDir::new("/tmp/dot_target", false).to_path_buf();
        let source = PathBuf::from("ferris.png");

        let mut dots = HashMap::new();

        dots.insert(
            "dot".to_string(),
            Dot {
                source: source.clone(),
                target: target.clone(),
            },
        );

        let config = Bombadil {
            path: PathBuf::from("tests/dotfiles_non_utf8")
                .canonicalize()
                .unwrap(),
            dots,
            vars: Variables::default(),
            hooks: vec![],
            profiles: Default::default(),
        };

        // Act
        config
            .traverse_dots_and_copy(
                &config.source_path(&source).unwrap(),
                &config.dot_copy_source_path(&source),
            )
            .unwrap();

        // Assert
        assert!(Path::new("tests/dotfiles_simple/.dots/template").exists());
    }

    #[test]
    fn should_return_dot_path() {
        // Arrange
        let config = Bombadil {
            path: PathBuf::from("tests/dotfiles_simple")
                .canonicalize()
                .unwrap(),
            dots: HashMap::new(),
            vars: Variables::default(),
            hooks: vec![],
            profiles: Default::default(),
        };

        // Act
        let path = config.dot_copy_source_path(&PathBuf::from("template"));

        // Assert
        assert!(path
            .to_str()
            .unwrap()
            .contains("tests/dotfiles_simple/.dots/template"));
        assert!(path.is_absolute());
    }

    #[test]
    fn self_link_works() {
        // Arrange
        let config_path = PathBuf::from("tests/dotfiles_simple/bombadil.toml");

        // Act
        Bombadil::link_self_config(Some(config_path)).unwrap();

        // Assert
        let link = dirs::config_dir().unwrap().join("bombadil.toml");
        assert!(link.exists());
    }

    #[test]
    fn install_single_file_works() {
        // Arrange
        let target = TempDir::new("/tmp/dot_target", false).to_path_buf();

        let mut map = HashMap::new();
        map.insert("red".to_string(), "red_value".to_string());

        let mut dots = HashMap::new();
        dots.insert(
            "dot".to_string(),
            Dot {
                source: PathBuf::from("template"),
                target: target.clone(),
            },
        );

        let config = Bombadil {
            path: PathBuf::from("tests/dotfiles_simple")
                .canonicalize()
                .unwrap(),
            dots,
            vars: Variables { variables: map },
            hooks: vec![],
            profiles: Default::default(),
        };

        // Act
        config.install().unwrap();

        // Assert
        assert!(target.exists());
        assert_eq!(
            std::fs::read_to_string(&target).unwrap(),
            "color: red_value".to_string()
        );
    }

    #[test]
    fn install_should_failsafely_and_continue() {
        // Arrange
        let target = TempDir::new("/tmp/dot_target", false).to_path_buf();

        let mut dots = HashMap::new();

        dots.insert(
            "dot".to_string(),
            Dot {
                source: PathBuf::from("template"),
                target: target.clone(),
            },
        );
        dots.insert(
            "dot".to_string(),
            Dot {
                source: PathBuf::from("invalid_path"),
                target: PathBuf::from("somewhere"),
            },
        );

        let config = Bombadil {
            path: PathBuf::from("tests/dotfiles_invalid_dot")
                .canonicalize()
                .unwrap(),
            dots,
            vars: Variables {
                variables: HashMap::new(),
            },
            hooks: vec![],
            profiles: Default::default(),
        };

        // Act
        config.install().unwrap();

        // Assert
        assert!(target.exists());
    }

    #[test]
    fn install_with_subdir() {
        // Arrange
        let target = TempDir::new("/tmp/sub_dir_target", false).to_path_buf();

        let mut map = HashMap::new();
        map.insert("red".to_string(), "red_value".to_string());
        map.insert("blue".to_string(), "blue_value".to_string());

        let mut dots = HashMap::new();

        dots.insert(
            "dot".to_string(),
            Dot {
                source: PathBuf::from("sub_dir"),
                target: target.clone(),
            },
        );

        let config = Bombadil {
            path: PathBuf::from("tests/dotfiles_nested")
                .canonicalize()
                .unwrap(),
            dots,
            vars: Variables { variables: map },
            hooks: vec![],
            profiles: Default::default(),
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
        let target = TempDir::new("/tmp/sub_dir_2_target", false).to_path_buf();

        let mut map = HashMap::new();
        map.insert("red".to_string(), "red_value".to_string());
        map.insert("blue".to_string(), "blue_value".to_string());

        let mut dots = HashMap::new();
        dots.insert(
            "dot".to_string(),
            Dot {
                source: PathBuf::from("sub_dir_1"),
                target: target.clone(),
            },
        );
        let config = Bombadil {
            path: PathBuf::from("tests/dotfiles_nested_2")
                .canonicalize()
                .unwrap(),
            dots,
            vars: Variables { variables: map },
            hooks: vec![],
            profiles: Default::default(),
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

    #[test]
    fn hook_ok() {
        // Arrange
        let target = TempDir::new("/tmp/hook", false).to_path_buf();
        let target_str_path = &target.to_str().unwrap();
        let config = Bombadil {
            path: PathBuf::from("tests/hook").canonicalize().unwrap(),
            dots: HashMap::new(),
            vars: Variables::default(),
            hooks: vec![Hook {
                command: format!("touch {}/dummy", target_str_path),
            }],
            profiles: Default::default(),
        };

        // Act
        config.install().unwrap();

        // Assert
        assert!(target.join("dummy").exists());
    }

    #[test]
    fn meta_var_works() {
        // Arrange
        let tmp = TempDir::new("/tmp/bombadil_tests", false).to_path_buf();
        // We need an absolute path to the test can pass anywhere
        std::fs::copy("tests/vars/meta_vars.toml", &tmp.join("meta_vars.toml")).unwrap();
        std::fs::copy("tests/vars/vars.toml", &tmp.join("vars.toml")).unwrap();
        std::fs::copy("tests/vars/bombadil.toml", &tmp.join("bombadil.toml")).unwrap();

        let config_path = tmp.join("bombadil.toml");

        Bombadil::link_self_config(Some(config_path.clone())).unwrap();

        // Act
        let bombadil = Bombadil::from_settings().unwrap();

        // Assert
        assert_eq!(
            bombadil.vars.variables.get("red"),
            Some(&"#FF0000".to_string())
        );
        assert_eq!(
            bombadil.vars.variables.get("black"),
            Some(&"#000000".to_string())
        );
        assert_eq!(
            bombadil.vars.variables.get("green"),
            Some(&"#008000".to_string())
        );

        let _ = std::fs::remove_dir_all(tmp);
    }

    #[test]
    fn should_symlink() {
        // Arrange
        let home = dirs::home_dir().unwrap();
        let tmp = TempDir::new("/tmp/test_link", false).to_path_buf();
        std::fs::copy("tests/dotfiles_simple/template", &tmp.join("template")).unwrap();

        // Act
        Bombadil::link(
            &PathBuf::from("/tmp/test_link/template"),
            &home.join("test_template"),
        );

        // Assert
        assert!(std::fs::symlink_metadata(&home.join("test_template")).is_ok());
        let _ = Bombadil::unlink(&home.join("test_template"));
    }

    #[test]
    fn should_unlink() {
        // Arrange
        let home = dirs::home_dir().unwrap();
        let tmp = TempDir::new("/tmp/test_link", false).to_path_buf();
        std::fs::copy("tests/dotfiles_simple/template", &tmp.join("template")).unwrap();
        Bombadil::link(
            &PathBuf::from("/tmp/test_link/template"),
            &home.join("test_template"),
        );

        // Act
        let result = Bombadil::unlink(&home.join("test_template"));

        // Assert
        assert!(result.is_ok());
        assert!(std::fs::symlink_metadata(home.join("test_template")).is_err());
    }
}
