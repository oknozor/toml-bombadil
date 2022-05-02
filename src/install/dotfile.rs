use std::convert::TryFrom;
use std::path::PathBuf;
use crate::{Dot, DotPaths, Variables};
use crate::error::*;

pub enum DotFile {
    Dir {
        // The directory source path
        source: PathBuf,
        // Symlink path
        target: PathBuf,
        // Dotfiles subdirectories and file
        children: Vec<DotFile>,
        // Local variables
        vars: Option<Variables>,
    },
    File {
        // The directory source path
        source: PathBuf,
        // Symlink path
        target: PathBuf,
        // Local variables
        vars: Option<Variables>,
    },
}

impl TryFrom<Dot> for DotFile {
    type Error = crate::error::Error;

    fn try_from(dot: Dot) -> Result<Self> {
        let source = dot.source()?;
        let target = dot.target()?;

        let vars = dot
            .resolve_var_path()
            .and_then(|path| Variables::from_toml(path.as_path()).ok());

        if source.is_dir() {
            Ok(DotFile::Dir {
                source,
                target,
                children: vec![],
                vars,
            })
        } else {
            Ok(DotFile::File {
                source,
                target,
                vars,
            })
        }
    }
}