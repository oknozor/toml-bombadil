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
    pub settings: ActiveProfile,

    #[serde(default)]
    pub profiles: HashMap<String, Profile>,

    /// Paths to merge with the main configuration
    #[serde(default)]
    pub import: Vec<ImportPath>,
}

impl Settings {
    pub(crate) fn merge_imports(&mut self) -> anyhow::Result<()> {
        let import_paths: Vec<PathBuf> = self
            .import
            .iter()
            .map(|import| import.path.clone())
            .map(|path| {
                if path.is_absolute() {
                    path
                } else {
                    self.get_dotfiles_path().unwrap().join(path)
                }
            })
            .collect();

        for path in import_paths.iter() {
            if path.exists() {
                let mut s = Config::new();
                s.merge(File::from(path.to_owned()))?;

                let sub_setting = s
                    .try_into::<ImportedSettings>()
                    .map_err(|err| anyhow!("{} : {}", "Config format error".red(), err));

                match sub_setting {
                    Ok(sub_settings) => self.merge(sub_settings),
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
        self.settings.dots.extend(sub_settings.settings.dots);
        self.profiles.extend(sub_settings.profiles);
    }
}
