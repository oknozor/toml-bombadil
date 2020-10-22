use crate::{unlink, Bombadil};
use anyhow::Result;
use colored::*;
use config::Config;
use config::File;
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;

const STATE_FILE: &str = "previous_state.toml";

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BombadilState {
    #[serde(skip)]
    pub path: PathBuf,
    pub symlinks: Vec<PathBuf>,
}

impl BombadilState {
    pub fn read(path: PathBuf) -> Result<Self> {
        let state_path = path.join(".dots").join(STATE_FILE);

        if state_path.exists() {
            let mut s = Config::new();
            s.merge(File::from(state_path))?;
            s.try_into()
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
        fs::write(&self.path, &content)?;
        Ok(())
    }

    pub fn remove_targets(&self) {
        self.symlinks.iter().for_each(|path| {
            let _ = unlink(path);
        });
    }
}

impl From<&Bombadil> for BombadilState {
    fn from(current: &Bombadil) -> Self {
        // Since we come from current bombadil config, unwrap is safe
        let path = current
            .dotfiles_absolute_path()
            .unwrap()
            .join(".dots")
            .join(STATE_FILE);
        let symlinks = current
            .dots
            .iter()
            .map(|dot| dot.1.target_path().unwrap())
            .collect();

        Self { path, symlinks }
    }
}
