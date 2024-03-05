use crate::paths::{unlink, DotPaths};
use crate::Bombadil;
use anyhow::{anyhow, Result};
use colored::*;
use config::Config;
use config::File;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;

const STATE_FILE: &str = "previous_state.toml";

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BombadilState {
    #[serde(skip)]
    pub path: PathBuf,
    pub symlinks: HashSet<PathBuf>,
}

impl BombadilState {
    pub fn read(path: PathBuf) -> Result<Self> {
        let state_path = path.join(".dots").join(STATE_FILE);

        if state_path.exists() {
            Config::builder()
                .add_source(File::from(state_path))
                .build()?
                .try_deserialize::<BombadilState>()
                .map_err(|err| anyhow!("{} : {}", "Previous state format error".red(), err))
        } else {
            Err(anyhow!(
                "Unable to find Previous state file {}",
                state_path.display()
            ))
        }
    }

    pub fn write(&self) -> Result<()> {
        let content = toml::to_string(&self)?;
        fs::write(&self.path, content)?;
        fs::File::open(&self.path)?.sync_data()?;
        Ok(())
    }

    pub fn remove_targets(&self) -> Vec<Result<PathBuf>> {
        let mut unlink_results = vec![];

        self.symlinks.iter().for_each(|path| {
            unlink_results.push(
                unlink(path)
                    .map(|_| path.to_owned())
                    .map_err(|err| anyhow!("Failed to unlink dot entry {:?} : {}", path, err)),
            );
        });

        unlink_results
    }
}

impl From<&mut Bombadil> for BombadilState {
    fn from(current: &mut Bombadil) -> Self {
        // Since we come from current bombadil settings, unwrap is safe
        let path = current
            .dotfiles_absolute_path()
            .unwrap()
            .join(".dots")
            .join(STATE_FILE);
        let symlinks = current
            .dots
            .iter()
            .map(|dot| dot.1.target().unwrap())
            .collect();

        Self { path, symlinks }
    }
}
