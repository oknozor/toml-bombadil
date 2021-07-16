// Fixme : This should not be needed when updating pest to the up coming release
#![allow(clippy::upper_case_acronyms)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate pest_derive;

use crate::dots::{Dot, DotVar};
use crate::gpg::Gpg;
use crate::hook::Hook;
use crate::settings::{Profile, Settings};
use crate::state::BombadilState;
use crate::templating::Variables;
use anyhow::Result;
use colored::*;
use std::collections::HashMap;
use std::fs;
use std::os::unix;
use std::path::{Path, PathBuf};

mod dots;
mod gpg;
mod hook;
pub mod settings;
mod state;
mod templating;

pub struct Bombadil {
    path: PathBuf,
    dots: HashMap<String, Dot>,
    vars: Variables,
    prehooks: Vec<Hook>,
    posthooks: Vec<Hook>,
    profiles: HashMap<String, Profile>,
    gpg: Option<Gpg>,
}

pub enum Mode {
    Gpg,
    NoGpg,
}

impl Bombadil {
    pub fn link_self_config(config_path: Option<PathBuf>) -> Result<()> {
        let xdg_config_dir = dirs::config_dir();
        if xdg_config_dir.is_none() {
            return Err(anyhow!("$XDG_CONFIG does not exist"));
        }

        let xdg_config = Settings::bombadil_config_xdg_path()?;

        if fs::symlink_metadata(&xdg_config).is_ok() {
            fs::remove_file(&xdg_config)?;
        }

        let config_path = &config_path
            .unwrap_or_else(|| PathBuf::from("bombadil.toml"))
            .canonicalize()?;

        let config_path = if config_path.is_dir() {
            config_path.join("bombadil.toml")
        } else {
            config_path.to_owned()
        };

        unix::fs::symlink(&config_path, &xdg_config)
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
        self.prehooks.iter().map(Hook::run).for_each(|result| {
            if let Err(err) = result {
                eprintln!("{}", err);
            }
        });
        let dot_copy_dir = &self.path.join(".dots");

        let absolute_path_to_dot = &self.dotfiles_absolute_path()?;

        // Get previous state if any and remove symlinks
        let previous_state = BombadilState::read(absolute_path_to_dot.to_owned());

        match previous_state {
            Ok(state) => {
                state.remove_targets();
                println!("{}", "Previous configuration cleaned up".green())
            }
            Err(err) => println!(
                "{} : {}",
                "No previous configuration found, skipping clean up".yellow(),
                err
            ),
        }

        if dot_copy_dir.exists() {
            fs::remove_dir_all(&dot_copy_dir)?;
        }

        // Render current config and create symlinks
        fs::create_dir(dot_copy_dir)?;
        for (key, dot) in self.dots.iter() {
            if let Err(err) = dot.install(
                absolute_path_to_dot,
                &self.vars,
                self.get_auto_ignored_files(key),
                self.gpg.as_ref(),
            ) {
                eprintln!("{}", err);
                continue;
            }

            dot.unlink()?;
            dot.symlink(absolute_path_to_dot)?;
        }

        // Run post install hooks
        self.posthooks.iter().map(Hook::run).for_each(|result| {
            if let Err(err) = result {
                eprintln!("{}", err);
            }
        });

        // Dump current config
        BombadilState::from(self).write()?;

        Ok(())
    }

    pub fn uninstall(&self) -> Result<()> {
        let mut success_paths: Vec<&PathBuf> = Vec::new();
        let mut error_paths: Vec<&anyhow::Error> = Vec::new();

        // Remove symlink from previous state
        let path = self.dotfiles_absolute_path()?;
        let previous_state = BombadilState::read(path)?;
        let remove_result = previous_state.remove_targets();

        remove_result
            .iter()
            .for_each(|remove_result| match remove_result {
                Ok(path) => success_paths.push(path),
                Err(e) => error_paths.push(e),
            });

        if !success_paths.is_empty() {
            println!("{}", "Removed symlinks:".green());
            success_paths.iter().for_each(|path| {
                let path_string = format!("\t{:?}", path).green();
                println!("{}", path_string);
            });
        }

        if !error_paths.is_empty() {
            println!("{}", "Error removing symlinks:".red());
            error_paths.iter().for_each(|path| {
                let path_string = format!("\t{:?}", path).red();
                println!("{}", path_string);
            });
        }

        Ok(())
    }

    pub fn add_secret<S: AsRef<Path> + ?Sized>(
        &self,
        key: &str,
        value: &str,
        var_file: &S,
    ) -> Result<()> {
        if let Some(gpg) = &self.gpg {
            gpg.push_secret(key, value, var_file)
        } else {
            Err(anyhow!("No gpg_user_id in bombadil config"))
        }
    }

    pub fn display_vars(&self) {
        self.vars
            .variables
            .iter()
            .for_each(|(key, value)| println!("{} = {}", key.red(), value))
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
                        dot.target = target.clone()
                    }

                    if let Some(vars) = &dot_override.vars {
                        dot.vars = vars.clone();
                    }

                    if let (None, None, None) = (
                        &dot_override.source,
                        &dot_override.target,
                        &dot_override.vars,
                    ) {
                        let warning = format!(
                            "Skipping {}, no `source`, `target` or `vars` to override",
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
                    let ignore = dot_override.ignore.clone();

                    self.dots.insert(
                        key.to_string(),
                        Dot {
                            source,
                            target,
                            ignore,
                            vars: Dot::default_vars(),
                        },
                    );
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
            let variables = Variables::from_paths(&self.path, &profile.vars, self.gpg.as_ref())?;
            self.vars.extend(variables);
            // Add Profile pre hooks
            let prehooks = profile
                .prehooks
                .iter()
                .map(|command| command.as_ref())
                .map(Hook::new)
                .collect::<Vec<Hook>>();
            self.prehooks.extend(prehooks);

            // Add profile post hooks
            let posthooks = profile
                .posthooks
                .iter()
                .map(|command| command.as_ref())
                .map(Hook::new)
                .collect::<Vec<Hook>>();
            self.posthooks.extend(posthooks);
        }

        Ok(())
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

    pub fn from_settings(mode: Mode) -> Result<Bombadil> {
        let config = Settings::get()?;
        let path = config.get_dotfiles_path()?;

        let gpg = match mode {
            Mode::Gpg => config.gpg_user_id.map(|user_id| Gpg::new(&user_id)),
            Mode::NoGpg => None,
        };

        // Resolve variables from path
        let mut vars = Variables::from_paths(&path, &config.settings.vars, gpg.as_ref())?;

        // Replace % reference with their ref value
        vars.resolve_ref();

        // Resolve hooks from config
        let posthooks = config
            .settings
            .posthooks
            .iter()
            .map(|cmd| Hook::new(cmd))
            .collect();

        let prehooks = config
            .settings
            .prehooks
            .iter()
            .map(|cmd| Hook::new(cmd))
            .collect();
        let dots = config.settings.dots;
        let profiles = config.profiles;

        Ok(Self {
            path,
            dots,
            vars,
            prehooks,
            posthooks,
            profiles,
            gpg,
        })
    }

    pub fn print_metadata(&self, metadata_type: MetadataType) {
        let rows = match metadata_type {
            MetadataType::Dots => self
                .dots
                .iter()
                .map(|(k, v)| {
                    format!(
                        "{}: {} => {}",
                        k,
                        self.path.join(&v.source).display(),
                        v.target_path()
                            .unwrap_or_else(|_| v.target.clone())
                            .display()
                    )
                })
                .collect(),
            MetadataType::Prehooks => self.prehooks.iter().map(|h| h.command.clone()).collect(),
            MetadataType::Posthooks => self.posthooks.iter().map(|h| h.command.clone()).collect(),
            MetadataType::Path => vec![self.path.display().to_string()],
            MetadataType::Profiles => {
                let mut profiles = vec!["default".to_string()];
                profiles.extend(self.profiles.iter().map(|(k, _)| k.clone()));
                profiles
            }
            MetadataType::Vars => self
                .vars
                .variables
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect(),
            MetadataType::Secrets => self
                .vars
                .secrets
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect(),
        };

        if !rows.is_empty() {
            println!("{}", rows.join("\n"));
        }
    }

    fn dotfiles_absolute_path(&self) -> Result<PathBuf> {
        dirs::home_dir()
            .ok_or_else(|| anyhow!("$HOME dir not found"))
            .map(|path| path.join(&self.path))
    }

    fn get_auto_ignored_files(&self, dot_key: &str) -> Vec<PathBuf> {
        let dot_origin = self.dots.get(dot_key);
        let origin_source = dot_origin.map(|dot| &dot.source);

        let mut ignored: Vec<PathBuf> = self
            .profiles
            .iter()
            .map(|(_, profile)| profile.dots.get(dot_key))
            .filter(Option::is_some)
            .map(|dot| dot.unwrap())
            .filter(|dot| dot.vars.is_some())
            .filter_map(|dot| dot.resolve_var_path(&self.path, origin_source))
            .collect();

        let _ = dot_origin.map(|dot| {
            dot.resolve_var_path(&self.path)
                .map(|path| ignored.push(path))
        });

        ignored
    }
}

pub(crate) fn unlink(path: &Path) -> Result<()> {
    if fs::symlink_metadata(path).is_ok() {
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}

pub enum MetadataType {
    Dots,
    Prehooks,
    Posthooks,
    Path,
    Profiles,
    Vars,
    Secrets,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dots::{DotOverride, DotVar};
    use crate::Mode::NoGpg;
    use std::collections::HashMap;
    use std::fs;
    use std::fs::read_link;
    use std::os::unix;
    use temp_testdir::TempDir;

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
                ignore: vec![],
                vars: Dot::default_vars(),
            },
        );

        let config = Bombadil {
            path: PathBuf::from("tests/dotfiles_simple")
                .canonicalize()
                .unwrap(),
            dots,
            vars: Variables {
                variables: map,
                secrets: Default::default(),
            },
            prehooks: vec![],
            posthooks: vec![],
            profiles: Default::default(),
            gpg: None,
        };

        // Act
        config.install().unwrap();

        // Assert
        assert!(target.exists());
        assert_eq!(
            fs::read_to_string(&target).unwrap(),
            "color: red_value".to_string()
        );
    }

    #[test]
    fn install_should_fail_and_continue() {
        // Arrange
        let target = TempDir::new("/tmp/dot_target", false).to_path_buf();

        let mut dots = HashMap::new();

        dots.insert(
            "dot".to_string(),
            Dot {
                source: PathBuf::from("template"),
                target: target.clone(),
                ignore: vec![],
                vars: Dot::default_vars(),
            },
        );
        dots.insert(
            "dot".to_string(),
            Dot {
                source: PathBuf::from("invalid_path"),
                target: PathBuf::from("somewhere"),
                ignore: vec![],
                vars: Dot::default_vars(),
            },
        );

        let config = Bombadil {
            path: PathBuf::from("tests/dotfiles_invalid_dot")
                .canonicalize()
                .unwrap(),
            dots,
            vars: Variables {
                variables: HashMap::new(),
                secrets: Default::default(),
            },
            prehooks: vec![],
            posthooks: vec![],
            profiles: Default::default(),
            gpg: None,
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
                ignore: vec![],
                vars: Dot::default_vars(),
            },
        );

        let config = Bombadil {
            path: PathBuf::from("tests/dotfiles_nested")
                .canonicalize()
                .unwrap(),
            dots,
            vars: Variables {
                variables: map,
                secrets: Default::default(),
            },
            prehooks: vec![],
            posthooks: vec![],
            profiles: Default::default(),
            gpg: None,
        };

        // Act
        config.install().unwrap();

        // Assert
        assert!(target.exists());
        let path = &target.read_link().unwrap();
        let red_dot = fs::read_to_string(path.join("template_1")).unwrap();
        let blue_dot = fs::read_to_string(path.join("template_2")).unwrap();
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
                ignore: vec![],
                vars: Dot::default_vars(),
            },
        );
        let config = Bombadil {
            path: PathBuf::from("tests/dotfiles_nested_2")
                .canonicalize()
                .unwrap(),
            dots,
            vars: Variables {
                variables: map,
                secrets: Default::default(),
            },
            prehooks: vec![],
            posthooks: vec![],
            profiles: Default::default(),
            gpg: None,
        };

        // Act
        config.install().unwrap();

        // Assert
        assert!(target.exists());
        let path = &target.read_link().unwrap();
        let red_dot = fs::read_to_string(path.join("template_1")).unwrap();
        let blue_dot = fs::read_to_string(path.join("subdir_2").join("template_2")).unwrap();
        assert_eq!(red_dot, "color: red_value".to_string());
        assert_eq!(blue_dot, "color: blue_value".to_string());
    }

    #[test]
    fn uninstall_works() {
        // Arrange
        let target = TempDir::new("/tmp/dot_unlink_target", false).to_path_buf();

        let mut dots = HashMap::new();
        dots.insert(
            "dot_1".to_string(),
            Dot {
                source: PathBuf::from("dot_1"),
                target: target.clone(),
                ignore: vec![],
                vars: Dot::default_vars(),
            },
        );

        let config = Bombadil {
            path: PathBuf::from("tests/dotfiles_unlink")
                .canonicalize()
                .unwrap(),
            dots,
            vars: Variables {
                variables: HashMap::new(),
                secrets: Default::default(),
            },
            prehooks: vec![],
            posthooks: vec![],
            profiles: Default::default(),
            gpg: None,
        };

        config.install().unwrap();

        // Act
        config.uninstall().unwrap();

        // Assert
        assert!(!target.exists());
    }

    #[test]
    fn posthook_ok() {
        // Arrange
        let target = TempDir::new("/tmp/hook", false).to_path_buf();
        let target_str_path = &target.to_str().unwrap();
        let config = Bombadil {
            path: PathBuf::from("tests/hook").canonicalize().unwrap(),
            dots: HashMap::new(),
            vars: Variables::default(),
            prehooks: vec![],
            posthooks: vec![Hook {
                command: format!("touch {}/dummy", target_str_path),
            }],
            profiles: Default::default(),
            gpg: None,
        };

        // Act
        config.install().unwrap();

        // Assert
        assert!(target.join("dummy").exists());
    }
    #[test]
    fn prehook_ok() {
        // Arrange
        let target = TempDir::new("/tmp/hook", false).to_path_buf();
        let target_str_path = &target.to_str().unwrap();
        let config = Bombadil {
            path: PathBuf::from("tests/hook").canonicalize().unwrap(),
            dots: HashMap::new(),
            vars: Variables::default(),
            prehooks: vec![Hook {
                command: format!("touch {}/dummy", target_str_path),
            }],
            posthooks: vec![],
            profiles: Default::default(),
            gpg: None,
        };

        // Act
        config.install().unwrap();

        // Assert
        assert!(target.join("dummy").exists());
        let target = TempDir::new("/tmp/hook", false).to_path_buf();
    }

    #[test]
    fn meta_var_works() {
        // Arrange
        let tmp = TempDir::new("/tmp/bombadil_tests", false).to_path_buf();
        // We need an absolute path to the test can pass anywhere
        fs::copy("tests/vars/meta_vars.toml", &tmp.join("meta_vars.toml")).unwrap();
        fs::copy("tests/vars/vars.toml", &tmp.join("vars.toml")).unwrap();
        fs::copy("tests/vars/bombadil.toml", &tmp.join("bombadil.toml")).unwrap();

        let config_path = tmp.join("bombadil.toml");

        Bombadil::link_self_config(Some(config_path.clone())).unwrap();

        // Act
        let bombadil = Bombadil::from_settings(NoGpg).unwrap();

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

        let _ = fs::remove_dir_all(tmp);
    }

    #[test]
    fn should_print_metadata() {
        // Arrange
        let tmp = TempDir::new("/tmp/bombadil_tests", false).to_path_buf();
        // We need an absolute path to the test can pass anywhere
        fs::copy("tests/vars/meta_vars.toml", &tmp.join("meta_vars.toml")).unwrap();
        fs::copy("tests/vars/vars.toml", &tmp.join("vars.toml")).unwrap();
        fs::copy("tests/vars/bombadil.toml", &tmp.join("bombadil.toml")).unwrap();

        let config_path = tmp.join("bombadil.toml");

        Bombadil::link_self_config(Some(config_path.clone())).unwrap();
        let bombadil = Bombadil::from_settings(NoGpg).unwrap();

        // Act
        bombadil.print_metadata(MetadataType::Dots);
        bombadil.print_metadata(MetadataType::Prehooks);
        bombadil.print_metadata(MetadataType::Posthooks);
        bombadil.print_metadata(MetadataType::Path);
        bombadil.print_metadata(MetadataType::Profiles);
        bombadil.print_metadata(MetadataType::Vars);

        // Assert
        // STDOUT should be asserted once those test facilities are in place.
        let _ = fs::remove_dir_all(tmp);
    }

    #[test]
    fn should_get_auto_ignored_files() -> Result<()> {
        let temp = TempDir::default();
        let temp = temp.to_path_buf();
        let source = temp.join("source");
        fs::create_dir(&source)?;

        let var_one = &source.join("vars_default.toml");
        let var_two = &source.join("vars_p1.toml");
        let var_three = &source.join("vars_p2.toml");

        fs::write(var_one, "1")?;
        fs::write(var_two, "1")?;
        fs::write(var_three, "1")?;

        let mut dots = HashMap::new();
        dots.insert(
            "dot".to_string(),
            Dot {
                source: source.clone(),
                target: Default::default(),
                ignore: vec![],
                vars: PathBuf::from("vars_default.toml"),
            },
        );

        let mut dots_profile_one = HashMap::new();
        dots_profile_one.insert(
            "dot".to_string(),
            DotOverride {
                source: Some(source.clone()),
                target: Default::default(),
                ignore: vec![],
                vars: Some(PathBuf::from("vars_p1.toml")),
            },
        );

        let mut dots_profile_two = HashMap::new();
        dots_profile_two.insert(
            "dot".to_string(),
            DotOverride {
                source: Some(source.clone()),
                target: Default::default(),
                ignore: vec![],
                vars: Some(PathBuf::from("vars_p2.toml")),
            },
        );

        let mut profiles = HashMap::new();
        profiles.insert(
            "profile_one".to_string(),
            Profile {
                dots: dots_profile_one,
                prehooks: vec![],
                posthooks: vec![],
                vars: vec![],
            },
        );

        profiles.insert(
            "profile_two".to_string(),
            Profile {
                dots: dots_profile_two,
                prehooks: vec![],
                posthooks: vec![],
                vars: vec![],
            },
        );

        let bombadil = Bombadil {
            path: temp,
            dots,
            vars: Default::default(),
            prehooks: vec![],
            posthooks: vec![],
            profiles,
            gpg: None,
        };

        let ignored = bombadil.get_auto_ignored_files("dot");

        println!("{:?}", ignored);
        assert!(ignored.contains(var_one));
        assert!(ignored.contains(var_two));
        assert!(ignored.contains(var_three));
        Ok(())
    }

    #[test]
    fn should_unlink_dir() -> Result<()> {
        let tmp = TempDir::default();
        let source = tmp.join("dir");
        let link = tmp.join("link");
        fs::create_dir(&source)?;

        unix::fs::symlink(&source, &link)?;
        assert_eq!(
            &read_link(&link)?.to_str().unwrap(),
            &source.to_str().unwrap()
        );

        unlink(&link)?;
        assert!(source.exists());
        assert!(!link.exists());

        Ok(())
    }

    #[test]
    fn should_unlink_file() -> Result<()> {
        let tmp = TempDir::default();
        let source = tmp.join("dir");
        let link = tmp.join("link");
        fs::write(&source, "Hello Tom")?;

        unix::fs::symlink(&source, &link)?;
        assert_eq!(
            &read_link(&link)?.to_str().unwrap(),
            &source.to_str().unwrap()
        );

        unlink(&link)?;
        assert!(source.exists());
        assert!(!link.exists());

        Ok(())
    }
}
