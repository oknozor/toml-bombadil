use crate::paths::DotPaths;
use crate::settings::dotfile_dir;
use crate::settings::dots::{Dot, DotOverride};
use crate::templating::Variables;
use anyhow::Result;
use colored::*;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(PartialEq, Eq, Debug)]
pub enum LinkResult {
    Updated,
    Created,
    Ignored,
    Unchanged,
}

impl Dot {
    pub(crate) fn install(
        &self,
        vars: &Variables,
        auto_ignored: Vec<PathBuf>,
    ) -> Result<LinkResult> {
        let source = &self.source()?;
        let target = &self.build_copy_path();
        let source_str = source.to_str().unwrap_or_default();

        let ignored_paths = if self.ignore.is_empty() {
            auto_ignored
        } else {
            let mut ignored_paths = self.get_ignored_paths(source_str)?;
            ignored_paths.extend_from_slice(&auto_ignored);
            ignored_paths
        };

        // Add local vars to the global ones
        let mut vars = vars.clone();

        if let Some(local_vars_path) = self.resolve_var_path() {
            let local_vars = Dot::load_local_vars(&local_vars_path);
            vars.extend(local_vars);
        }

        // Resolve % reference
        vars.resolve_ref();

        // Recursively copy dotfile to .dots directory
        self.traverse_and_copy(source, target, ignored_paths.as_slice(), &vars)
    }

    fn load_local_vars(source: &Path) -> Variables {
        Variables::from_toml(source).unwrap_or_else(|err| {
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
        source: &PathBuf,
        target: &PathBuf,
        ignored: &[PathBuf],
        vars: &Variables,
    ) -> Result<LinkResult> {
        if ignored.contains(source) {
            return Ok(LinkResult::Ignored);
        }

        // Single file : inject vars and write to .dots/
        if source.is_file() {
            fs::create_dir_all(&target.parent().unwrap())?;
            match vars.to_dot(source) {
                Ok(content) if target.exists() => self.update(source, target, content),
                Ok(content) => self.create(source, target, content),
                Err(_) if target.exists() => {
                    // Fixme: we probabbly want to remove anyhow here
                    // And handle specific error case (i.e: ignore utf8 error)
                    self.update_raw(source, target)
                }
                Err(_) => {
                    fs::copy(source, target)?;
                    Ok(LinkResult::Created)
                }
            }
        } else {
            fs::create_dir_all(target)?;
            let mut link_results = vec![];

            for entry in source.read_dir()? {
                let entry_path = &entry?.path();
                let entry_name = entry_path.file_name().unwrap().to_str().unwrap();
                let result = self.traverse_and_copy(
                    &source.join(entry_name),
                    &target.join(entry_name),
                    ignored,
                    vars,
                );

                match result {
                    Ok(result) => link_results.push(result),
                    Err(err) => eprintln!("{err}"),
                }
            }

            if link_results.contains(&LinkResult::Updated) {
                Ok(LinkResult::Updated)
            } else if link_results.contains(&LinkResult::Created) {
                Ok(LinkResult::Created)
            } else {
                Ok(LinkResult::Unchanged)
            }
        }
    }

    fn create(&self, source: &PathBuf, target: &PathBuf, content: String) -> Result<LinkResult> {
        let permissions = fs::metadata(source)?.permissions();
        let mut dot_copy = File::create(&target)?;
        dot_copy.write_all(content.as_bytes())?;
        dot_copy.set_permissions(permissions)?;
        Ok(LinkResult::Created)
    }

    fn update(&self, source: &PathBuf, target: &PathBuf, content: String) -> Result<LinkResult> {
        let target_content = fs::read_to_string(target)?;
        if target_content == content {
            Ok(LinkResult::Unchanged)
        } else {
            let permissions = fs::metadata(source)?.permissions();
            let mut dot_copy = OpenOptions::new().write(true).truncate(true).open(target)?;
            dot_copy.write_all(content.as_bytes())?;
            dot_copy.set_permissions(permissions)?;
            dot_copy.sync_data()?;
            Ok(LinkResult::Updated)
        }
    }

    fn update_raw(&self, source: &PathBuf, target: &PathBuf) -> Result<LinkResult> {
        let target_content = fs::read(target)?;
        let content = fs::read(source)?;

        if target_content == content {
            Ok(LinkResult::Unchanged)
        } else {
            let permissions = fs::metadata(source)?.permissions();
            let mut dot_copy = OpenOptions::new().write(true).truncate(true).open(target)?;

            dot_copy.write_all(&content)?;
            dot_copy.set_permissions(permissions)?;
            dot_copy.sync_data()?;
            Ok(LinkResult::Updated)
        }
    }
}

impl DotOverride {
    pub(crate) fn resolve_var_path(&self, origin: Option<&PathBuf>) -> Option<PathBuf> {
        let source = match (self.get_source(), origin) {
            (Some(source), _) => source,
            (None, Some(origin)) => origin,
            _ => panic!("Dot has no source path"),
        };

        let vars = self.vars().unwrap_or_else(Dot::default_vars);
        self.resolve_from_source(source, &vars)
    }
}

impl Dot {}

pub(crate) trait DotVar {
    fn vars(&self) -> Option<PathBuf>;
    fn get_source(&self) -> Option<&PathBuf>;
    fn default_vars() -> PathBuf {
        PathBuf::from("vars.toml")
    }

    fn is_default_var_path(&self) -> bool {
        self.vars() == Some(Dot::default_vars())
    }

    fn resolve_from_source(&self, source: &Path, path: &Path) -> Option<PathBuf> {
        let relative_to_dot = dotfile_dir().join(source).join(path);
        let relative_to_dotfile_dir = dotfile_dir().join(path);
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
                self.vars_path_not_found(source, path)
            }
        } else {
            // Warning is emitted only if the path is not "vars.toml"
            self.vars_path_not_found(source, path)
        }
    }

    fn vars_path_not_found(&self, source: &Path, path: &Path) -> Option<PathBuf> {
        if !self.is_default_var_path() {
            eprintln!(
                "{} {:?} {} {:?} {} {:?}",
                "WARNING: Variable path".yellow(),
                path,
                "was neither found in".yellow(),
                source,
                "nor in".yellow(),
                dotfile_dir()
            );
        }
        None
    }
}

impl DotVar for Dot {
    fn vars(&self) -> Option<PathBuf> {
        Some(self.vars.clone())
    }

    fn get_source(&self) -> Option<&PathBuf> {
        Some(&self.source)
    }
}

impl DotVar for DotOverride {
    fn vars(&self) -> Option<PathBuf> {
        self.vars.clone()
    }

    fn get_source(&self) -> Option<&PathBuf> {
        self.source.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::dots::DotVar;
    use crate::settings::dots::Dot;
    use crate::templating::Variables;
    use crate::Mode::NoGpg;
    use crate::{Bombadil, DotPaths};
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
            target: PathBuf::from(".settings/sway"),
            ignore: vec![],
            vars: Dot::default_vars(),
        };

        // Act
        let result = dot.target();

        // Assert
        assert_that!(result)
            .is_ok()
            .is_equal_to(PathBuf::from(home).join(".settings/sway"));
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
        let result = dot.target();

        // Assert
        assert_that!(result)
            .is_ok()
            .is_equal_to(PathBuf::from("/etc/profile"));
    }

    #[sealed_test(files = ["tests/dotfiles_simple"], before = setup("dotfiles_simple"))]
    fn symlink_ok() -> Result<()> {
        // Arrange
        run_cmd!(
            mkdir dotfiles_simple/.dots;
            echo "Hello Tom" > dotfiles_simple/.dots/template.css;
        )?;

        let dot = Dot {
            source: PathBuf::from("template.css"),
            target: PathBuf::from(".config/template.css"),
            ignore: vec![],
            vars: Dot::default_vars(),
        };

        // Act
        dot.symlink()?;

        // Assert
        let symlink = PathBuf::from(".config/template.css");
        assert_that!(symlink.is_symlink()).is_true();
        assert_that!(fs::read_to_string(symlink)?).is_equal_to(&"Hello Tom\n".to_string());

        Ok(())
    }

    #[sealed_test(files = ["tests/dotfiles_with_multiple_nested_dir"], before = setup("dotfiles_with_multiple_nested_dir"))]
    fn copy() -> Result<()> {
        // Arrange
        let dot = Dot {
            source: PathBuf::from("dir"),
            target: PathBuf::from(".config/dir"),
            ignore: vec![],
            vars: Dot::default_vars(),
        };

        // Act
        dot.traverse_and_copy(
            &PathBuf::from("dotfiles_with_multiple_nested_dir/dir"),
            &PathBuf::from("dotfiles_with_multiple_nested_dir/.dots/dir"),
            &vec![],
            &Variables::default(),
        )?;

        run_cmd!(tree -a;)?;

        // Assert
        let file_one =
            PathBuf::from("dotfiles_with_multiple_nested_dir/.dots/dir/subdir_one/subfile");
        let file_two =
            PathBuf::from("dotfiles_with_multiple_nested_dir/.dots/dir/subdir_two/subfile");
        assert_that!(file_one).exists();
        assert_that!(file_two).exists();
        assert_that!(fs::read_to_string(file_one)?).is_equal_to(&"Hello From subdir 1".to_string());
        assert_that!(fs::read_to_string(file_two)?).is_equal_to(&"Hello From subdir 2".to_string());
        Ok(())
    }

    #[sealed_test(files = ["tests/dotfiles_non_utf8"], before = setup("dotfiles_non_utf8"))]
    fn copy_non_utf8() -> Result<()> {
        let source = PathBuf::from("dotfiles_non_utf8/ferris.png");
        let target = PathBuf::from("dotfiles_non_utf8/.config/ferris.png");

        let dot = Dot {
            source: source.clone(),
            target,
            ignore: vec![],
            vars: Dot::default_vars(),
        };

        dot.traverse_and_copy(
            &source,
            &PathBuf::from("dotfiles_non_utf8/.dots/ferris.png"),
            &vec![],
            &Variables::default(),
        )?;

        run_cmd!(tree -a;)?;

        assert_that!(PathBuf::from("dotfiles_non_utf8/.dots/ferris.png")).exists();
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

        dot.symlink()?;

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

        dot.install(&Variables::default(), vec![])?;

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
        dot.install(&vars, vec![])?;
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

        dot.install(&Variables::default(), vec![])?;

        let content = fs::read_to_string(".dots/dir/template")?;
        assert_that!(content).is_equal_to(&"Hello Tom\n".to_string());

        Ok(())
    }

    #[sealed_test(files = ["tests/dotfiles_with_local_vars"], env = [("HOME", ".")])]
    fn install_with_local_vars_default_path() -> Result<()> {
        run_cmd!(
            mkdir .config;
            mkdir dotfiles_with_local_vars/source_dot;
            echo "{{name}} is {{verb}}" > dotfiles_with_local_vars/source_dot/file;
            echo "name=\"Tom\"" > dotfiles_with_local_vars/source_dot/vars.toml;
            echo "verb=\"singing\"" >> dotfiles_with_local_vars/source_dot/vars.toml;
        )?;

        Bombadil::link_self_config(Some(PathBuf::from(
            "dotfiles_with_local_vars/bombadil.toml",
        )))?;

        let dot = Dot {
            source: PathBuf::from("source_dot"),
            target: PathBuf::from("target_dot"),
            ignore: vec![],
            // FIXME: this should be relative to the dotfile directory
            vars: PathBuf::from("dotfiles_with_local_vars/source_dot/vars.toml"),
        };

        // Arrange
        dot.install(&Variables::default(), vec![])?;

        // Assert
        let content = fs::read_to_string(PathBuf::from(
            "dotfiles_with_local_vars/.dots/source_dot/file",
        ))?;
        assert_eq!(content, "Tom is singing\n");
        Ok(())
    }
}
