use crate::gpg::Gpg;
use crate::templating::Variables;
use crate::unlink;
use anyhow::{anyhow, Result};
use colored::*;
use dirs::home_dir;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::unix;
use std::path::{Path, PathBuf};

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
    #[serde(skip_serializing)]
    pub ignore: Vec<String>,
    // A single var file attached to the dot
    #[serde(default = "Dot::default_vars")]
    #[serde(skip_serializing)]
    pub vars: PathBuf,
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
    // A single var file attached to the dot
    pub vars: Option<PathBuf>,
}

impl Dot {
    pub(crate) fn install<P: AsRef<Path>>(
        &self,
        dotfile_dir: P,
        vars: &Variables,
        auto_ignored: Vec<PathBuf>,
        gpg: Option<&Gpg>,
    ) -> Result<()> {
        let source = &self.source_path(dotfile_dir.as_ref())?;
        let copy_path = &self.copy_path(dotfile_dir.as_ref());
        let source_str = source.to_str().unwrap_or_default();
        let mut ignored_paths = self.get_ignored_paths(source_str)?;
        ignored_paths.extend_from_slice(&auto_ignored);

        // Add local vars to the global ones
        let mut vars = vars.clone();

        if let Some(local_vars_path) = self.resolve_var_path(dotfile_dir.as_ref()) {
            let local_vars = Dot::load_local_vars(&local_vars_path, gpg);
            vars.extend(local_vars);
        }

        // Resolve % reference
        vars.resolve_ref();

        // Recursively copy dotfile to .dots directory
        self.traverse_and_copy(source, copy_path, ignored_paths.as_slice(), &vars)
    }

    pub(crate) fn symlink<P: AsRef<Path>>(&self, dotfile_dir: P) -> Result<()> {
        let copy_path = &self.copy_path(dotfile_dir.as_ref());
        let target = &self.target_path()?;

        // Link
        unix::fs::symlink(copy_path, target)
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
        unlink(target)
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

    fn load_local_vars(source: &Path, gpg: Option<&Gpg>) -> Variables {
        Variables::from_toml(source, gpg).unwrap_or_else(|err| {
            eprintln!("{}", err.to_string().yellow());
            Variables::default()
        })
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

    fn traverse_and_copy(
        &self,
        source: &Path,
        target: &Path,
        ignored: &[PathBuf],
        vars: &Variables,
    ) -> Result<()> {
        if ignored.contains(&source.to_path_buf()) {
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
    fn source_path(&self, dotfile_dir: &Path) -> Result<PathBuf> {
        let path = dotfile_dir.join(&self.source);

        if path.exists() {
            Ok(path)
        } else {
            Err(anyhow!(format!(
                "{} {:?}",
                "Path does not exist :".red(),
                path
            )))
        }
    }

    pub(crate) fn copy_path<P: AsRef<Path>>(&self, dotfile_dir: P) -> PathBuf {
        dotfile_dir.as_ref().join(".dots").join(&self.source)
    }
}

impl DotOverride {
    pub(crate) fn resolve_var_path(
        &self,
        dotfile_dir: &Path,
        origin: Option<&PathBuf>,
    ) -> Option<PathBuf> {
        let source = match (self.source(), origin) {
            (Some(source), _) => source,
            (None, Some(origin)) => origin,
            _ => panic!("Dot has no source path"),
        };

        let vars = self.vars().unwrap_or_else(Dot::default_vars);
        self.resolve_from_source(dotfile_dir, source, &vars)
    }
}

impl Dot {
    pub(crate) fn resolve_var_path(&self, dotfile_dir: &Path) -> Option<PathBuf> {
        self.resolve_from_source(dotfile_dir, &self.source, &self.vars)
    }
}

pub(crate) trait DotVar {
    fn vars(&self) -> Option<PathBuf>;
    fn source(&self) -> Option<&PathBuf>;
    fn default_vars() -> PathBuf {
        PathBuf::from("vars.toml")
    }

    fn is_default_var_path(&self) -> bool {
        self.vars() == Some(Dot::default_vars())
    }

    fn resolve_from_source(
        &self,
        dotfile_dir: &Path,
        source: &Path,
        path: &Path,
    ) -> Option<PathBuf> {
        let relative_to_dot = dotfile_dir.join(source).join(path);
        let relative_to_dotfile_dir = dotfile_dir.join(path);
        // FIXME : we should not try to look for path like this
        // Instead "../vars.toml" should be used
        if relative_to_dot.exists() {
            Some(relative_to_dot)
        } else if let Some(parent) = source.parent() {
            if parent.join(path).exists() {
                Some(parent.join(path))
            } else if relative_to_dotfile_dir.exists() && !self.is_default_var_path() {
                Some(relative_to_dotfile_dir)
            } else {
                self.vars_path_not_found(dotfile_dir, source, path)
            }
        } else {
            // Warning is emitted only if the path is not "vars.toml"
            self.vars_path_not_found(dotfile_dir, source, path)
        }
    }

    fn vars_path_not_found(
        &self,
        dotfile_dir: &Path,
        source: &Path,
        path: &Path,
    ) -> Option<PathBuf> {
        if !self.is_default_var_path() {
            eprintln!(
                "{} {:?} {} {:?} {} {:?}",
                "WARNING: Variable path".yellow(),
                path,
                "was neither found in".yellow(),
                source,
                "nor in".yellow(),
                dotfile_dir
            );
        }
        None
    }
}

impl DotVar for Dot {
    fn vars(&self) -> Option<PathBuf> {
        Some(self.vars.clone())
    }

    fn source(&self) -> Option<&PathBuf> {
        Some(&self.source)
    }
}

impl DotVar for DotOverride {
    fn vars(&self) -> Option<PathBuf> {
        self.vars.clone()
    }

    fn source(&self) -> Option<&PathBuf> {
        self.source.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::dots::{Dot, DotVar};
    use crate::templating::Variables;
    use crate::Bombadil;
    use crate::Mode::NoGpg;
    use anyhow::Result;
    use cmd_lib::{init_builtin_logger, run_cmd};
    use sealed_test::prelude::*;
    use speculoos::prelude::*;
    use std::path::PathBuf;
    use std::{env, fs};

    fn setup(dotfiles: &str) {
        env::set_var("HOME", env::current_dir().unwrap());
        init_builtin_logger();
        run_cmd!(
            mkdir .config;
            tree -a;
        )
        .unwrap();

        Bombadil::link_self_config(Some(PathBuf::from(dotfiles))).unwrap();
    }

    #[sealed_test]
    fn should_get_target_path() {
        // Arrange
        let home = env!("HOME");

        let dot = Dot {
            source: Default::default(),
            target: PathBuf::from(".config/sway"),
            ignore: vec![],
            vars: Dot::default_vars(),
        };

        // Act
        let result = dot.target_path();

        // Assert
        assert_that!(result)
            .is_ok()
            .is_equal_to(PathBuf::from(home).join(".config/sway"));
    }

    #[sealed_test]
    fn should_get_absolute_target_path() {
        // Arrange
        let dot = Dot {
            source: Default::default(),
            target: PathBuf::from("/etc/profile"),
            ignore: vec![],
            vars: Dot::default_vars(),
        };

        // Act
        let result = dot.target_path();

        // Assert
        assert_that!(result)
            .is_ok()
            .is_equal_to(PathBuf::from("/etc/profile"));
    }

    #[sealed_test(env = [("HOME", ".")])]
    fn symlink_ok() -> Result<()> {
        // Arrange
        run_cmd!(
            mkdir .dots;
            echo "Hello Tom" > .dots/dot;
        )?;

        let dot = Dot {
            source: PathBuf::from("dot"),
            target: PathBuf::from("dot_target"),
            ignore: vec![],
            vars: Dot::default_vars(),
        };

        // Act
        dot.symlink(".")?;
        run_cmd!(tree -a;)?;

        // Assert
        let symlink = PathBuf::from("dot_target");
        assert_that!(symlink.is_symlink()).is_true();
        assert_that!(fs::read_to_string(symlink)?).is_equal_to(&"Hello Tom\n".to_string());

        Ok(())
    }

    #[sealed_test(env = [("HOME", ".")])]
    fn copy() -> Result<()> {
        // Arrange
        run_cmd!(
            mkdir -p dir/subdir_one;
            mkdir -p dir/subdir_two;
            mkdir .dots;
            echo "Hello From subdir 1" > dir/subdir_one/subfile;
            echo "Hello From subdir 2" > dir/subdir_two/subfile;
        )?;

        let dot = Dot {
            source: PathBuf::from("dir"),
            target: PathBuf::from("."),
            ignore: vec![],
            vars: Dot::default_vars(),
        };

        // Act
        dot.traverse_and_copy(
            PathBuf::from("dir").canonicalize()?.as_path(),
            PathBuf::from(".dots/dir").as_path(),
            &vec![],
            &Variables::default(),
        )?;

        // Assert
        let file_one = PathBuf::from(".dots/dir/subdir_one/subfile");
        let file_two = PathBuf::from(".dots/dir/subdir_two/subfile");
        assert_that!(file_one).exists();
        assert_that!(file_two).exists();
        assert_that!(fs::read_to_string(file_one)?)
            .is_equal_to(&"Hello From subdir 1\n".to_string());
        assert_that!(fs::read_to_string(file_two)?)
            .is_equal_to(&"Hello From subdir 2\n".to_string());
        Ok(())
    }

    #[sealed_test(files = ["tests/dotfiles_non_utf8/ferris.png"],env = [("HOME", ".")])]
    fn copy_non_utf8() -> Result<()> {
        run_cmd!(tree -a;)?;

        let source = PathBuf::from("ferris.png");
        let target = PathBuf::from("ferris.png");

        let dot = Dot {
            source: source.clone(),
            target,
            ignore: vec![],
            vars: Dot::default_vars(),
        };

        dot.traverse_and_copy(
            &source,
            PathBuf::from(".dots/ferris.png").as_path(),
            &vec![],
            &Variables::default(),
        )?;

        assert_that!(PathBuf::from(".dots/ferris.png")).exists();
        Ok(())
    }

    #[sealed_test(env = [("HOME", ".")])]
    fn copy_with_ignore() -> Result<()> {
        // Arrange
        run_cmd!(
            mkdir -p source_dot/subdir_one;
            mkdir -p source_dot/subdir_two;
            mkdir .dots;

            echo "Not Hello Tom" > source_dot/file.md;
            echo "Hello Tom" > source_dot/file;

            echo "Hello From subdir 2" > source_dot/subdir_two/subfile;
            echo "Ignored" > source_dot/subdir_two/subfile.md;
        )?;

        let dot = Dot {
            source: PathBuf::from("source_dot"),
            target: PathBuf::from("source_dot"),
            ignore: vec!["*.md".to_string()],
            vars: Dot::default_vars(),
        };

        // Act
        dot.traverse_and_copy(
            &PathBuf::from("source_dot").canonicalize()?,
            &PathBuf::from(".dots/source_dot"),
            &vec![
                PathBuf::from("source_dot/subdir_two/subfile.md"),
                PathBuf::from("source_dot/file.md"),
            ],
            &Variables::default(),
        )?;

        // Assert

        let file_content = fs::read_to_string(".dots/source_dot/file")?;

        assert_that!(file_content).is_equal_to(&"Hello Tom\n".to_string());
        assert_that!(PathBuf::from(".dots/source_dot/subdir_one")).exists();
        assert_that!(PathBuf::from(".dots/source_dot/subdir_two")).exists();

        let file_content = fs::read_to_string(".dots/source_dot/subdir_two/subfile")?;

        assert_that!(file_content).is_equal_to(&"Hello From subdir 2\n".to_string());
        assert_that!(PathBuf::from(".dots/file.md")).does_not_exist();
        assert_that!(PathBuf::from(".dots/subdir_two/subfile.md")).does_not_exist();
        Ok(())
    }

    #[sealed_test(env = [("HOME", ".")])]
    fn unlink() -> Result<()> {
        // Arrange
        let source = PathBuf::from("source_dot");
        let target = PathBuf::from("target_dot");

        fs::create_dir(".dots")?;
        fs::write(".dots/source_dot", "Hello Tom")?;

        let dot = Dot {
            source,
            target,
            ignore: vec![],
            vars: Dot::default_vars(),
        };

        dot.symlink(".")?;

        // Act
        dot.unlink()?;

        // Assert
        let target = dirs::home_dir().unwrap().join("target_dot");

        assert_that!(target).does_not_exist();

        Ok(())
    }

    #[sealed_test(env = [("HOME", ".")])]
    fn install() -> Result<()> {
        run_cmd!(echo "source" > source_dot;)?;

        let source = PathBuf::from("source_dot");
        let target = PathBuf::from("target_dot");

        let dot = Dot {
            source,
            target,
            ignore: vec![],
            vars: Dot::default_vars(),
        };

        dot.install(env::current_dir()?, &Variables::default(), vec![], None)?;

        assert_that!(PathBuf::from(".dots")).exists();
        assert_that!(PathBuf::from(".dots/source_dot")).exists();
        Ok(())
    }

    #[sealed_test(env = [("HOME", ".")])]
    fn install_with_vars() -> Result<()> {
        run_cmd!(
            mkdir dotfiles;
            echo "Hello {{name}}" > dotfiles/dot;
        )?;

        let dot = Dot {
            source: PathBuf::from("dotfiles/dot"),
            target: PathBuf::from("dot"),
            ignore: vec![],
            vars: Dot::default_vars(),
        };

        let mut vars = Variables::default();
        vars.insert("name", "Tom Bombadil");

        // Act
        dot.install(env::current_dir()?, &vars, vec![], None)?;
        let dot = PathBuf::from(".dots/dotfiles/dot");

        // Assert
        assert_that!(dot).exists();
        assert_that!(fs::read_to_string(dot)?).is_equal_to(&"Hello Tom Bombadil\n".to_string());
        Ok(())
    }

    #[sealed_test(files = ["tests/dotfiles_with_local_vars"], before = setup("dotfiles_with_local_vars"))]
    fn install_with_local_vars() -> Result<()> {
        let bombadil = Bombadil::from_settings(NoGpg)?;
        bombadil.install()?;

        let dotfiles = bombadil.dots.get("sub_dir").unwrap();
        assert_that!(dotfiles.vars).is_equal_to(PathBuf::from("vars.toml"));

        let content = fs::read_to_string("dotfiles_with_local_vars/.dots/sub_dir/template")?;
        assert_that!(content).is_equal_to(&"Golberry is singing".to_string());
        Ok(())
    }

    #[sealed_test(env = [("HOME", ".")])]
    fn install_with_local_vars_dot_relative() -> Result<()> {
        run_cmd!(
            mkdir dir;
            echo "Hello {{name}}" > dir/template;
            echo "name=\"Tom\"" > dir/my_vars.toml;
        )?;

        let dot = Dot {
            source: PathBuf::from("dir"),
            target: PathBuf::from("dir"),
            ignore: vec![],
            vars: PathBuf::from("my_vars.toml"),
        };

        dot.install(env::current_dir()?, &Variables::default(), vec![], None)?;

        let content = fs::read_to_string(".dots/dir/template")?;
        assert_that!(content).is_equal_to(&"Hello Tom\n".to_string());

        Ok(())
    }

    #[sealed_test(files = ["tests/dotfiles_with_local_vars"])]
    fn install_with_local_vars_default_path() -> Result<()> {
        run_cmd!(
            mkdir source_dot;
            echo "{{name}} is {{verb}}" > source_dot/file;
            echo "name=\"Tom\"" > source_dot/vars.toml;
            echo "verb=\"singing\"" >> source_dot/vars.toml;
        )?;

        let dot = Dot {
            source: PathBuf::from("source_dot"),
            target: PathBuf::from("target_dot"),
            ignore: vec![],
            vars: PathBuf::from("vars.toml"),
        };

        // Arrange
        dot.install(env::current_dir()?, &Variables::default(), vec![], None)?;

        // Assert
        let content = fs::read_to_string(PathBuf::from(".dots/source_dot/file"))?;
        assert_eq!(content, "Tom is singing\n");
        Ok(())
    }
}
