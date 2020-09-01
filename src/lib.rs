#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate pest_derive;

use crate::dots::{Dot, Profile, ProfileSwitch};
use crate::hook::Hook;
use crate::settings::Settings;
use crate::templating::Variables;
use anyhow::Result;
use colored::*;
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
    dots: Vec<Dot>,
    vars: Variables,
    hooks: Vec<Hook>,
}

impl Bombadil {
    pub fn link_self_config(config_path: Option<PathBuf>) -> Result<()> {
        let xdg_config_dir = dirs::config_dir();
        if xdg_config_dir.is_none() {
            return Err(anyhow!("$XDG_CONFIG does not exists",));
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

        let dot_copy_dir = &self.path.join(".dots");

        if dot_copy_dir.exists() {
            std::fs::remove_dir_all(&dot_copy_dir)?;
        }

        std::fs::create_dir(dot_copy_dir)?;

        for dot in self.dots.iter() {
            let dot_source_path = self.source_path(&dot.source);

            if let Err(err) = dot_source_path {
                eprintln!("{}", err);
                continue;
            }

            let dot_copy_path = self.dot_copy_source_path(&dot.source);

            self.traverse_dots_and_copy(&dot_source_path?, &dot_copy_path)?;

            let target = &dot.target()?;

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

    pub fn from_settings() -> Result<Bombadil> {
        let settings = Settings::get()?;

        let home_dir = dirs::home_dir();
        if home_dir.is_none() {
            return Err(anyhow!("$HOME directory not found"));
        }

        let base_dir = if settings.dotfiles_dir.is_absolute() {
            settings.dotfiles_dir
        } else {
            home_dir.unwrap().join(&settings.dotfiles_dir)
        };

        if base_dir.exists().not() {
            return Err(anyhow!(
                "Dotfiles directory {:?} does not exists",
                &base_dir
            ));
        }

        // Get meta variables from config
        let mut meta_vars = Variables::default();
        if let Some(settings_meta_vars) = settings.meta {
            for var in settings_meta_vars {
                let variables = Variables::from_toml(&base_dir.join(var.path))?;
                meta_vars.extend(variables);
            }
        }

        // Resolve variables from path
        let mut vars = Variables::default();
        if let Some(setting_vars) = settings.var {
            for var in setting_vars {
                let variables = Variables::from_toml(&base_dir.join(var.path))?;
                vars.extend(variables);
            }
        }

        let var_copy = vars.variables.clone();

        var_copy
            .iter()
            .filter(|var| meta_vars.variables.get(var.1).is_some())
            .for_each(|var| {
                let _ = vars.variables.insert(
                    var.0.to_string(),
                    meta_vars.variables.get(var.1).unwrap().to_string(),
                );
            });

        // Resolve hooks from config
        let mut hooks = vec![];
        if let Some(setting_hooks) = settings.hook {
            hooks.extend(setting_hooks);
        }

        Ok(Self {
            path: base_dir,
            dots: settings.dot,
            vars,
            hooks,
        })
    }

    pub fn update_profile(&mut self, dot_name: &str, profile_name: &str) -> Result<()> {
        let dot_with_profiles: &Dot = self
            .dots
            .iter()
            .filter(|dot| dot.profile.is_some() && dot.name.is_some())
            .find(|dot| dot.name.as_ref().unwrap() == dot_name)
            .as_ref()
            .unwrap();

        let dot_with_profiles = dot_with_profiles.clone();

        if profile_name == "default" {
            return self.set_profile(dot_with_profiles, None);
        }

        let selected_profile: &Profile = dot_with_profiles
            .profile
            .as_ref()
            .unwrap()
            .iter()
            .find(|p| p.name == profile_name)
            .as_ref()
            .unwrap();

        let selected_profile = selected_profile.clone();

        self.set_profile(dot_with_profiles, Some(selected_profile))
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

    fn remove_file_or_dir(target: &PathBuf) {
        if target.is_dir() {
            let _ = std::fs::remove_dir_all(&target);
        } else {
            let _ = std::fs::remove_file(&target);
        }
    }

    fn set_profile(&mut self, mut dot: Dot, profile: Option<Profile>) -> Result<()> {
        // Remove the previous symlink
        let target = &dot.target()?;
        Bombadil::unlink(target)?;

        // Remove the previous dot copy
        let dot_copy_path = &self.dot_copy_source_path(&dot.source);
        Bombadil::remove_file_or_dir(dot_copy_path);

        // Back to default profile
        if profile.is_none() {
            let dot_copy_path = self.dot_copy_source_path(&dot.source);
            let source_path = self.source_path(&dot.source)?;

            self.traverse_dots_and_copy(&source_path, &dot_copy_path)?;
            Bombadil::link(&dot_copy_path, target);
            return Ok(());
        }

        let profile = profile.unwrap();

        // Mutate the dot state before relinking (either vars or source path)
        match &profile.switch {
            ProfileSwitch::Vars(var_path) => {
                let var_path = self.source_path(var_path)?;
                let variables = Variables::from_toml(&var_path)?;
                self.vars.extend(variables)
            }

            ProfileSwitch::Source(source_path) => {
                dot.source = source_path.to_owned();
            }
        }

        // Write updated .dots and relink
        let dot_copy_path = self.dot_copy_source_path(&dot.source);
        let source_path = self.source_path(&dot.source)?;

        self.traverse_dots_and_copy(&source_path, &dot_copy_path)?;
        Bombadil::link(&dot_copy_path, target);

        if let Some(command) = profile.hook {
            let hook = Hook { command };
            if let Err(err) = hook.run() {
                let err = format!("{}", err).red();
                eprintln!("{}", err);
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

        let source = PathBuf::from("template");
        let config = Bombadil {
            path: PathBuf::from("tests/dotfiles_simple")
                .canonicalize()
                .unwrap(),
            dots: vec![Dot {
                name: None,
                source: source.clone(),
                target: target.clone(),
                profile: None,
            }],
            vars: Variables::default(),
            hooks: vec![],
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
        let config = Bombadil {
            path: PathBuf::from("tests/dotfiles_non_utf8")
                .canonicalize()
                .unwrap(),
            dots: vec![Dot {
                name: None,
                source: source.clone(),
                target: target.clone(),
                profile: None,
            }],
            vars: Variables::default(),
            hooks: vec![],
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
            dots: vec![],
            vars: Variables::default(),
            hooks: vec![],
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

        let config = Bombadil {
            path: PathBuf::from("tests/dotfiles_simple")
                .canonicalize()
                .unwrap(),
            dots: vec![Dot {
                name: None,
                source: PathBuf::from("template"),
                target: target.clone(),
                profile: None,
            }],
            vars: Variables { variables: map },
            hooks: vec![],
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

        let config = Bombadil {
            path: PathBuf::from("tests/dotfiles_invalid_dot")
                .canonicalize()
                .unwrap(),
            dots: vec![
                Dot {
                    name: None,
                    source: PathBuf::from("template"),
                    target: target.clone(),
                    profile: None,
                },
                Dot {
                    name: None,
                    source: PathBuf::from("invalid_path"),
                    target: PathBuf::from("somewhere"),
                    profile: None,
                },
            ],
            vars: Variables {
                variables: HashMap::new(),
            },
            hooks: vec![],
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

        let config = Bombadil {
            path: PathBuf::from("tests/dotfiles_nested")
                .canonicalize()
                .unwrap(),
            dots: vec![Dot {
                name: None,
                source: PathBuf::from("sub_dir"),
                target: target.clone(),
                profile: None,
            }],
            vars: Variables { variables: map },
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
        let target = TempDir::new("/tmp/sub_dir_2_target", false).to_path_buf();

        let mut map = HashMap::new();
        map.insert("red".to_string(), "red_value".to_string());
        map.insert("blue".to_string(), "blue_value".to_string());

        let config = Bombadil {
            path: PathBuf::from("tests/dotfiles_nested_2")
                .canonicalize()
                .unwrap(),
            dots: vec![Dot {
                name: None,
                source: PathBuf::from("sub_dir_1"),
                target: target.clone(),
                profile: None,
            }],
            vars: Variables { variables: map },
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

    #[test]
    fn hook_ok() {
        // Arrange
        let target = TempDir::new("/tmp/hook", false).to_path_buf();
        let target_str_path = &target.to_str().unwrap();
        let config = Bombadil {
            path: PathBuf::from("tests/hook").canonicalize().unwrap(),
            dots: vec![],
            vars: Variables::default(),
            hooks: vec![Hook {
                command: format!("touch {}/dummy", target_str_path),
            }],
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
    fn should_update_profile_vars() {
        // Arrange
        let tmp = TempDir::new("/tmp/bombadil_tests_var_profile", false).to_path_buf();
        // We need an absolute path to the test can pass anywhere
        std::fs::copy("tests/var_profile/dot", &tmp.join("dot")).unwrap();
        std::fs::copy(
            "tests/var_profile/profile-vars.toml",
            &tmp.join("profile-vars.toml"),
        )
        .unwrap();
        std::fs::copy(
            "tests/var_profile/default-vars.toml",
            &tmp.join("default-vars.toml"),
        )
        .unwrap();
        std::fs::copy(
            "tests/var_profile/bombadil.toml",
            &tmp.join("bombadil.toml"),
        )
        .unwrap();

        let config_path = tmp.join("bombadil.toml");

        Bombadil::link_self_config(Some(config_path.clone())).unwrap();
        let mut bombadil = Bombadil::from_settings().unwrap();
        let _ = bombadil.install();

        let content =
            std::fs::read_to_string(PathBuf::from(tmp.join(".dots").join("dot"))).unwrap();

        assert_eq!(content, "24");

        // Act
        let result = bombadil.update_profile("dot_name", "profile_name");

        // Assert
        assert!(result.is_ok());
        let content =
            std::fs::read_to_string(PathBuf::from(tmp.join(".dots").join("dot"))).unwrap();

        assert_eq!(content, "42");

        let _ = std::fs::remove_dir_all(tmp);
    }

    #[test]
    fn should_update_profile_source() {
        // Arrange
        let tmp = TempDir::new("/tmp/bombadil_tests_source_profile", false).to_path_buf();
        // We need an absolute path to the test can pass anywhere
        std::fs::copy("tests/source_profile/dot", &tmp.join("dot")).unwrap();
        std::fs::copy("tests/source_profile/alt_dot", &tmp.join("alt_dot")).unwrap();
        std::fs::copy(
            "tests/source_profile/bombadil.toml",
            &tmp.join("bombadil.toml"),
        )
        .unwrap();

        let config_path = tmp.join("bombadil.toml");

        Bombadil::link_self_config(Some(config_path.clone())).unwrap();
        let mut bombadil = Bombadil::from_settings().unwrap();
        let _ = bombadil.install();

        let content =
            std::fs::read_to_string(PathBuf::from(tmp.join(".dots").join("dot"))).unwrap();

        assert_eq!(content, "24");

        // Act
        let result = bombadil.update_profile("dot_name", "profile_name");

        // Assert
        assert!(result.is_ok());
        let content =
            std::fs::read_to_string(PathBuf::from(tmp.join(".dots").join("alt_dot"))).unwrap();

        assert_eq!(content, "42");

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

    #[test]
    fn should_remove_file() {
        // Arrange
        let tmp = TempDir::new("/tmp/test_remove", false).to_path_buf();
        let path = PathBuf::from("/tmp/test_remove/file");
        std::fs::File::create(&path).unwrap();
        assert!(path.exists());

        // Act
        Bombadil::remove_file_or_dir(&path);

        // Assert
        assert!(path.exists().not());
        let _ = std::fs::remove_dir(tmp);
    }

    #[test]
    fn should_remove_dir() {
        // Arrange
        let tmp = TempDir::new("/tmp/test_remove", false).to_path_buf();
        let path = PathBuf::from("/tmp/test_remove/dir");
        std::fs::create_dir(&path).unwrap();
        assert!(path.exists());

        // Act
        Bombadil::remove_file_or_dir(&path);

        // Assert
        assert!(path.exists().not());
        let _ = std::fs::remove_dir(tmp);
    }
}
