use crate::settings::profiles::ActiveProfile;
use crate::settings::Settings;
use crate::Profile;
use anyhow::anyhow;
use colored::Colorize;
use config::{Config, File};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ImportPath {
    path: PathBuf,
}

/// An imported configuration, same as `Settings` but without `dotfiles_dir`
#[derive(Debug, Deserialize, Serialize)]
pub struct ImportedSettings {
    #[serde(default)]
    pub source_paths_are_relative: bool,

    #[serde(skip)]
    pub source_path: PathBuf,

    #[serde(default)]
    pub settings: ActiveProfile,

    #[serde(default)]
    pub profiles: HashMap<String, Profile>,

    /// Paths to merge with the main configuration
    #[serde(default)]
    pub import: Vec<ImportPath>,
}

impl Settings {
    pub(crate) fn merge_imports(&mut self) -> anyhow::Result<()> {
        let mut absolute_paths: HashMap<PathBuf, bool> = HashMap::new();
        let import_paths: Vec<PathBuf> = self
            .import
            .iter()
            .map(|import| import.path.clone())
            .map(|path| {
                if path.is_absolute() {
                    absolute_paths.insert(path.to_owned(), true);
                    path
                } else {
                    self.get_dotfiles_path().unwrap().join(path)
                }
            })
            .collect();

        for path in import_paths.iter() {
            if path.exists() {
                let sub_setting: Result<ImportedSettings, anyhow::Error> = Config::builder()
                    .add_source(File::from(path.as_path()))
                    .build()?
                    .try_deserialize()
                    .map_err(|err| anyhow!("{} : {}", "Config format error".red(), err));

                match sub_setting {
                    Ok(mut sub_settings) => {
                        if absolute_paths.contains_key(path)
                            && sub_settings.source_paths_are_relative
                        {
                            eprintln!(
                                "{} Cannot import file \"{}\" as relative when specified as an absolute in the parent config",
                                "Error".red(),
                                path.display(),
                            );
                            continue;
                        }
                        if sub_settings.source_paths_are_relative {
                            sub_settings.source_path = path
                                .strip_prefix(self.get_dotfiles_path().unwrap())
                                .unwrap()
                                .to_owned();
                        }
                        if sub_settings.settings.run_hooks_in_dotfiles_dir {
                            return Err(anyhow!(
                                "Cannot import file \"{}\" with run_hooks_in_dotfiles_dir set to true, must be set in root config",
                                path.display()
                            ));
                        }
                        self.merge(sub_settings)
                    }
                    Err(err) => {
                        eprintln!("Error loading settings from : {:?} {}", path, err)
                    }
                }
            } else {
                eprintln!(
                    "{} {}",
                    "Unable to find bombadil import file".red(),
                    path.display()
                );
            }
        }

        Ok(())
    }

    fn merge(&mut self, sub_settings: ImportedSettings) {
        self.settings
            .prehooks
            .extend_from_slice(&sub_settings.settings.prehooks);
        self.settings
            .posthooks
            .extend_from_slice(&sub_settings.settings.posthooks);
        self.settings
            .vars
            .extend_from_slice(&sub_settings.settings.vars);
        self.import.extend_from_slice(&sub_settings.import);
        for (key, value) in sub_settings.settings.dots.iter() {
            let mut key = key.clone();
            let mut dot_with_sub_path = value.clone();
            if sub_settings.source_paths_are_relative {
                let parent_path = sub_settings.source_path.parent().unwrap();
                key = parent_path.join(key).to_str().unwrap().to_owned();
                dot_with_sub_path.source = parent_path.join(&value.source);
                println!(
                    "Overwriting source_path {} to {:?}",
                    &value.source.display(),
                    dot_with_sub_path.source
                );
            }
            if self.settings.dots.contains_key(&key) {
                eprintln!(
                    "{} {}",
                    "Duplicate key in imports \"{}\", skipping".red(),
                    key
                );
                continue;
            }
            if let Some(hard_copy_target) = &dot_with_sub_path.hard_copy_target {
                println!("Hard copy target {:?}", hard_copy_target);
            }
            self.settings.dots.insert(key.to_owned(), dot_with_sub_path);
        }
        self.profiles.extend(sub_settings.profiles);
    }
}
