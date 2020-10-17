use crate::templating::Variables;
use anyhow::Result;
use colored::*;
use dirs::home_dir;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs as unixfs;
use std::path::PathBuf;

/// Represent a link between a `source` dotfile in the user defined dotfiles directory
/// and the XDG `target` path where it should be linked
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Dot {
    /// Path relative to user defined dotfile
    pub source: PathBuf,
    /// Target path either relative to $HOME or absolute
    pub target: PathBuf,
    /// Glob pattern of files to ignore when creating symlinks
    #[serde(default)]
    pub ignore: Vec<String>,
}

/// Same as dot but source and target are optionals
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DotOverride {
    /// Path relative to user defined dotfile
    pub source: Option<PathBuf>,
    /// Target path either relative to $HOME or absolute
    pub target: Option<PathBuf>,
    /// Glob pattern of files to ignore when creating symlinks
    #[serde(default)]
    pub ignore: Vec<String>,
}

impl Dot {
    pub(crate) fn install(&self, dotfile_dir: &PathBuf, vars: &Variables) -> Result<()> {
        let source = &self.source_path(dotfile_dir)?;
        let target = &self.copy_path(dotfile_dir);
        let source_str = source.to_str().unwrap_or_default();
        let ignored_paths = self.get_ignored_paths(&source_str)?;

        self.traverse_and_copy(source, target, ignored_paths.as_slice(), vars)
    }

    fn get_ignored_paths(&self, source_str: &str) -> Result<Vec<PathBuf>> {
        Ok(
            globwalk::GlobWalkerBuilder::from_patterns(source_str, self.ignore.as_slice())
                .build()?
                .into_iter()
                .filter_map(Result::ok)
                .map(|entry| entry.path().to_path_buf())
                .collect(),
        )
    }

    pub(crate) fn symlink(&self, dotfile_dir: &PathBuf) -> Result<()> {
        let copy_path = &self.copy_path(dotfile_dir);
        let target = &self.target_path()?;

        // Link
        unixfs::symlink(copy_path, target)
            .map(|_result| {
                let source = format!("{:?}", copy_path).blue();
                let dest = format!("{:?}", target).green();
                println!("{} => {}", source, dest)
            })
            .map_err(|err| {
                let source = format!("{:?}", copy_path).blue();
                let dest = format!("{:?}", &target).red();
                let err = format!("{}", err).red().bold();
                anyhow!("{} => {} : {}", source, dest, err)
            })
            .unwrap_or_else(|err| eprintln!("{}", err));

        Ok(())
    }

    pub(crate) fn unlink(&self) -> Result<()> {
        let target = &self.target_path()?;
        if fs::symlink_metadata(target).is_ok() {
            // TODO REMOVE SYMLINK TOO
            if target.is_dir() {
                fs::remove_dir_all(target)?;
            } else {
                fs::remove_file(target)?;
            }
        }

        Ok(())
    }

    /// Return the target path of a dot entry either absolute or relative to $HOME
    pub(crate) fn target_path(&self) -> Result<PathBuf> {
        if self.target.is_absolute() {
            Ok(self.target.clone())
        } else {
            home_dir()
                .map(|home| home.join(&self.target))
                .ok_or_else(|| {
                    let err = format!("Unable to find dot path : {:?}", &self.target).red();
                    anyhow!(err)
                })
        }
    }

    fn traverse_and_copy(
        &self,
        source: &PathBuf,
        target: &PathBuf,
        ignored: &[PathBuf],
        vars: &Variables,
    ) -> Result<()> {
        if ignored.contains(source) {
            return Ok(());
        }

        // Single file : inject vars and write to .dots/
        if source.is_file() {
            fs::create_dir_all(&target.parent().unwrap())?;
            if let Ok(content) = vars.to_dot(source) {
                let permissions = fs::metadata(source)?.permissions();
                let mut dot_copy = File::create(&target)?;
                dot_copy.write_all(content.as_bytes())?;
                dot_copy.set_permissions(permissions)?;
            } else {
                // Something went wrong parsing or reading the source path,
                // We just copy the file in place
                fs::copy(source, target)?;
            }
        } else if source.is_dir() {
            fs::create_dir_all(target)?;
            for entry in source.read_dir()? {
                let entry_path = &entry?.path();
                let entry_name = entry_path.file_name().unwrap().to_str().unwrap();
                self.traverse_and_copy(
                    &source.join(entry_name),
                    &target.join(entry_name),
                    ignored,
                    vars,
                )
                .unwrap_or_else(|err| eprintln!("{}", err));
            }
        }
        Ok(())
    }

    /// Resolve dot source copy path ({dotfiles/dotsource) against user defined dotfile directory
    /// Check if file exists
    fn source_path(&self, dotfile_dir: &PathBuf) -> Result<PathBuf> {
        let path = dotfile_dir.join(&self.source);

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

    fn copy_path(&self, dotfile_dir: &PathBuf) -> PathBuf {
        dotfile_dir.join(".dots").join(&self.source)
    }
}

#[cfg(test)]
mod tests {
    use crate::dots::Dot;
    use crate::templating::Variables;
    use anyhow::Result;
    use std::fs;
    use std::path::PathBuf;
    use temp_testdir::TempDir;

    #[test]
    fn should_get_target_path() {
        // Arrange
        let home = env!("HOME");

        let dot = Dot {
            source: Default::default(),
            target: PathBuf::from(".config/sway"),
            ignore: vec![],
        };

        // Act
        let result = dot.target_path();

        // Assert
        assert!(result.is_ok());
        let expected = PathBuf::from(home).join(".config").join("sway");

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn should_get_absolute_target_path() {
        // Arrange
        let dot = Dot {
            source: Default::default(),
            target: PathBuf::from("/etc/profile"),
            ignore: vec![],
        };

        // Act
        let result = dot.target_path();

        // Assert
        assert!(result.is_ok());

        let expected = PathBuf::from("/etc/profile");
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn symlink_ok() -> Result<()> {
        // Arrange
        let temp = TempDir::default();
        let temp = &temp.to_path_buf();

        let source = PathBuf::from("source_dot");
        let target = PathBuf::from("target_dot");

        fs::create_dir(temp.join(".dots"))?;
        fs::write(&temp.join(".dots").join(&source), "Hello Tom")?;

        let dot = Dot {
            source,
            target,
            ignore: vec![],
        };

        // Act
        dot.symlink(temp)?;

        // Assert
        let target = dirs::home_dir().unwrap().join("target_dot");

        assert!(target.exists());
        assert_eq!(fs::read_to_string(target)?, "Hello Tom");

        Ok(dot.unlink()?)
    }

    #[test]
    fn copy() -> Result<()> {
        // Arrange
        let temp = TempDir::default();
        let temp = &temp.to_path_buf();

        let source = &PathBuf::from("source_dot");
        let target = PathBuf::from("target_dot");

        let absolute_source_path = &temp.join(&source);

        fs::create_dir(absolute_source_path)?;
        fs::write(absolute_source_path.join("file"), "Hello Tom")?;
        fs::create_dir_all(absolute_source_path.join("dir1").join("subdir_one"))?;
        fs::create_dir_all(absolute_source_path.join("dir1").join("subdir_two"))?;
        fs::write(
            absolute_source_path
                .join("dir1")
                .join("subdir_two")
                .join("subfile"),
            "Hello From subdir 2",
        )?;
        fs::create_dir(absolute_source_path.join("dir2"))?;

        let dot = Dot {
            source: source.clone(),
            target: target.clone(),
            ignore: vec![],
        };

        let absolute_source_path = dot.source_path(temp)?;

        // Act
        dot.traverse_and_copy(
            &absolute_source_path,
            &dot.copy_path(temp),
            &vec![],
            &Variables::default(),
        )?;

        // Assert
        let dots_copy_path = temp.join(".dots").join(source);
        let file_content = fs::read_to_string(dots_copy_path.join("file"))?;

        assert_eq!(file_content, "Hello Tom");
        assert!(dots_copy_path.join("dir1").exists());
        assert!(dots_copy_path.join("dir1/subdir_one").exists());
        assert!(dots_copy_path.join("dir1/subdir_two").exists());

        let file_content = fs::read_to_string(dots_copy_path.join("dir1/subdir_two/subfile"))?;
        assert_eq!(file_content, "Hello From subdir 2");
        assert!(dots_copy_path.join("dir2").exists());
        Ok(())
    }

    #[test]
    fn copy_non_utf8() -> Result<()> {
        let temp = TempDir::default();
        let temp = &temp.to_path_buf();

        let source = &PathBuf::from("ferris.png");
        let target = PathBuf::from("target_dot");

        let absolute_source_path = &temp.join(&source);
        fs::copy("tests/dotfiles_non_utf8/ferris.png", absolute_source_path)?;

        let dot = Dot {
            source: source.clone(),
            target: target.clone(),
            ignore: vec![],
        };

        dot.traverse_and_copy(
            &absolute_source_path,
            &dot.copy_path(temp),
            &vec![],
            &Variables::default(),
        )?;

        let dots_copy_path = temp.join(".dots").join(source);
        assert!(dots_copy_path.exists());
        Ok(())
    }

    #[test]
    fn copy_with_ignore() -> Result<()> {
        // Arrange
        let temp = TempDir::default();
        let temp = &temp.to_path_buf();

        let source = &PathBuf::from("source_dot");
        let target = PathBuf::from("target_dot");

        let absolute_source_path = &temp.join(&source);

        fs::create_dir(absolute_source_path)?;
        fs::write(absolute_source_path.join("file.md"), "Not Hello Tom")?;
        fs::write(absolute_source_path.join("file"), "Hello Tom")?;
        fs::create_dir_all(absolute_source_path.join("dir1").join("subdir_one"))?;
        fs::create_dir_all(absolute_source_path.join("dir1").join("subdir_two"))?;
        fs::write(
            absolute_source_path
                .join("dir1")
                .join("subdir_two")
                .join("subfile"),
            "Hello From subdir 2",
        )?;
        fs::write(
            absolute_source_path
                .join("dir1")
                .join("subdir_two")
                .join("subfile.md"),
            "Ignored",
        )?;
        fs::create_dir(absolute_source_path.join("dir2"))?;

        let dot = Dot {
            source: source.clone(),
            target: target.clone(),
            ignore: vec!["*.md".to_string()],
        };

        let absolute_source_path = dot.source_path(temp)?;
        let ignored_one = absolute_source_path.join("dir1/subdir_two/subfile.md");
        let ignored_two = absolute_source_path.join("file.md");

        // Act
        dot.traverse_and_copy(
            &absolute_source_path,
            &dot.copy_path(temp),
            &vec![ignored_one, ignored_two],
            &Variables::default(),
        )?;

        // Assert
        let dots_copy_path = temp.join(".dots").join(source);

        let file_content = fs::read_to_string(dots_copy_path.join("file"))?;

        assert_eq!(file_content, "Hello Tom");
        assert!(dots_copy_path.join("dir1").exists());
        assert!(dots_copy_path.join("dir1/subdir_one").exists());
        assert!(dots_copy_path.join("dir1/subdir_two").exists());

        let file_content = fs::read_to_string(dots_copy_path.join("dir1/subdir_two/subfile"))?;
        assert_eq!(file_content, "Hello From subdir 2");
        assert!(dots_copy_path.join("dir2").exists());

        let ignored_target_one = dots_copy_path.join("file.md");
        assert!(!ignored_target_one.exists());
        let ignored_target_two = dots_copy_path.join("dir1/subdir_two/subfile.md");
        assert!(!ignored_target_two.exists());
        Ok(())
    }

    #[test]
    fn unlink() -> Result<()> {
        // Arrange
        let temp = TempDir::default();
        let temp = &temp.to_path_buf();

        let source = PathBuf::from("source_dot");
        let target = PathBuf::from("target_dot");

        fs::create_dir(temp.join(".dots"))?;
        fs::write(&temp.join(".dots").join(&source), "Hello Tom")?;

        let dot = Dot {
            source,
            target,
            ignore: vec![],
        };

        dot.symlink(temp)?;

        // Act
        dot.unlink()?;

        // Assert
        let target = dirs::home_dir().unwrap().join("target_dot");

        assert!(!target.exists());

        Ok(())
    }

    #[test]
    fn install() -> Result<()> {
        let temp = TempDir::default();
        let temp = &temp.to_path_buf();

        let source = &PathBuf::from("source_dot");
        let target = PathBuf::from("target_dot");

        let absolute_source_path = &temp.join(&source);

        fs::create_dir(absolute_source_path)?;
        fs::write(absolute_source_path.join("file"), "Hello Tom")?;

        let dot = Dot {
            source: source.clone(),
            target: target.clone(),
            ignore: vec![],
        };

        dot.install(temp, &Variables::default())?;

        assert!(temp.join(".dots").exists());
        assert!(temp.join(".dots/source_dot").exists());
        assert!(temp.join(".dots/source_dot/file").exists());
        Ok(())
    }

    #[test]
    fn install_with_vars() -> Result<()> {
        let temp = TempDir::default();
        let temp = &temp.to_path_buf();

        let source = &PathBuf::from("source_dot");
        let target = PathBuf::from("target_dot");

        let absolute_source_path = &temp.join(&source);

        fs::create_dir(absolute_source_path)?;
        fs::write(absolute_source_path.join("file"), "Hello __[name]__")?;

        let dot = Dot {
            source: source.clone(),
            target: target.clone(),
            ignore: vec![],
        };

        let mut vars = Variables::default();
        vars.insert("name", "Tom Bombadil");

        dot.install(temp, &Variables::default())?;

        assert!(temp.join(".dots").exists());
        assert!(temp.join(".dots/source_dot").exists());
        assert!(temp.join(".dots/source_dot/file").exists());
        Ok(())
    }
}
