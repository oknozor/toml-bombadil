use self::settings::profiles::Profile;
use crate::dots::{DotVar, LinkResult};
use crate::gpg::Gpg;
use crate::hook::Hook;
use crate::paths::{unlink, DotPaths};
use crate::state::BombadilState;
use crate::templating::Variables;
use anyhow::{anyhow, Context, Result};
use colored::*;
use ignore_files::IgnoreFilter;
use settings::dots::Dot;
use settings::Settings;
use std::collections::HashMap;
use std::io::Write;
use std::os::unix;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use std::{fs, io};
use watchexec::{
    action::{Action, Outcome},
    config::{InitConfig, RuntimeConfig},
    error::RuntimeError,
    event::{filekind::FileEventKind, Tag},
    handler::PrintDebug,
    signal::source::MainSignal,
    Watchexec,
};
use watchexec_filterer_ignore::IgnoreFilterer;

mod dots;
mod error;
mod git;
mod gpg;
mod hook;
pub mod paths;
pub mod settings;
mod state;
mod templating;

pub(crate) const BOMBADIL_CONFIG: &str = "bombadil.toml";

/// The main crate struct, it contains all needed medata about a
/// dotfile directory and how to install it.
#[derive(Clone)]
pub struct Bombadil {
    // path to self configuration, relative to $HOME
    path: PathBuf,
    // A list of dotfiles to link for this instance
    dots: HashMap<String, Dot>,
    // Variables for the tera template context
    vars: Variables,
    // Pre-hook commands, run before `bombadil-link`
    prehooks: Vec<Hook>,
    // Post-hook commands, run after `bombadil-link`
    posthooks: Vec<Hook>,
    // Available profiles
    profiles: HashMap<String, Profile>,
    // Profiles enabled for this isntance
    profile_enabled: Vec<String>,
    // A GPG user id, linking to user encryption/decryption key via gnupg
    gpg: Option<Gpg>,
}

/// Enable or disable GPG encryption when linking dotfiles
pub enum Mode {
    Gpg,
    NoGpg,
}

impl Bombadil {
    /// Given a git remote address, will clone the repository to the target path
    /// and install the dotfiles according to the "bombadil.toml" configuration inside the
    /// repo root.
    pub fn install_from_remote(
        remote: &str,
        path: PathBuf,
        profiles: Option<Vec<&str>>,
    ) -> Result<()> {
        git::clone(remote, path.as_path())?;
        Bombadil::link_self_config(Some(path.join(BOMBADIL_CONFIG)))?;

        let mut bombadil = Bombadil::from_settings(Mode::Gpg)?;

        if let Some(profiles) = profiles {
            bombadil.enable_profiles(profiles)?;
        }

        bombadil.install()?;

        Ok(())
    }

    /// Symlink `bombadil.toml` to `$XDG_CONFIG/bombadil.toml` so we can later read it from there.
    pub fn link_self_config(dotfiles_path: Option<PathBuf>) -> Result<()> {
        // Get the provided path and attempt to resolve 'bombadil.toml' if it's a directory
        let path = match dotfiles_path {
            None => PathBuf::from(BOMBADIL_CONFIG),
            Some(path) if path.is_dir() => path.join(BOMBADIL_CONFIG),
            Some(path) => path,
        };

        match path.canonicalize() {
            Ok(path) => {
                match dirs::config_dir() {
                    None => Err(anyhow!("$XDG_CONFIG does not exist")),
                    Some(config_dir) => {
                        let bombadil_xdg_config = config_dir.join(BOMBADIL_CONFIG);

                        // Attempt to locate a previous '$HOME/.settings/bombadil.toml' link and remove it
                        if fs::symlink_metadata(&bombadil_xdg_config).is_ok() {
                            fs::remove_file(&bombadil_xdg_config)?;
                        }

                        // Symlink to '$HOME/.settings/bombadil.toml'
                        unix::fs::symlink(&path, &bombadil_xdg_config)
                            .map_err(|err| {
                                anyhow!(
                                    "Failed to symlink {:?} to {:?} : {}",
                                    path,
                                    bombadil_xdg_config,
                                    err
                                )
                            })
                            .map(|_result| {
                                let source = format!("{:?}", &path).blue();
                                let dest = format!("{:?}", &bombadil_xdg_config).green();
                                println!("{} => {}", source, dest)
                            })
                    }
                }
            }
            Err(_err) => {
                let err = format!("{path:?} {}", "not found in current directory");
                Err(anyhow!("{}", err.red()))
            }
        }
    }

    /// The installation process is composed of the following steps :
    /// 1. Run pre install hooks
    /// 2. If any previous state is found in `.dot/previous_state.toml`, remove the existing symlinks
    /// 3. Clean existing rendered dotfiles templates in `.dot`
    /// 4. Copy and symlink dotfiles according to the current `$XDG_CONFIG/bombadil.toml` configuration
    /// 5. Run post install hooks
    /// 6. Write current state to `.dot/previous_state.toml`
    pub fn install(&self) -> Result<()> {
        self.check_dotfile_dir()?;

        self.prehooks.iter().map(Hook::run).for_each(|result| {
            if let Err(err) = result {
                eprintln!("{}", err);
            }
        });
        let dot_copy_dir = &self.path.join(".dots");

        // Render current settings and create symlinks
        fs::create_dir_all(dot_copy_dir)?;
        for (key, dot) in self.dots.iter() {
            match dot.install(
                &self.vars,
                self.get_auto_ignored_files(key),
                self.profile_enabled.as_slice(),
            ) {
                Err(err) => {
                    eprintln!("{}", err);
                    continue;
                }
                Ok(linked) => {
                    let copy_path = &dot.copy_path()?;
                    let target = &dot.target()?;

                    match linked {
                        LinkResult::Updated => {
                            let source = format!("{:?}", copy_path).blue();
                            let dest = format!("{:?}", target).yellow();
                            println!("{} => {}", source, dest)
                        }
                        LinkResult::Created => {
                            let source = format!("{:?}", copy_path).blue();
                            let dest = format!("{:?}", target).green();
                            println!("Created - {} => {}", source, dest)
                        }
                        LinkResult::Ignored => {
                            let source = format!("{:?}", copy_path);
                            let dest = format!("{:?}", target);
                            println!("Ignored - {} => {}", source, dest)
                        }
                        LinkResult::Unchanged => {
                            let source = format!("{:?}", copy_path);
                            let dest = format!("{:?}", target);
                            println!("Unchanged - {} => {}", source, dest)
                        }
                    }
                }
            }

            dot.symlink()?;
        }

        // Run post install hooks
        self.posthooks.iter().map(Hook::run).for_each(|result| {
            if let Err(err) = result {
                eprintln!("Failed to run posthook: {}", err);
            }
        });

        // Dump current settings
        let absolute_path_to_dot = &self.dotfiles_absolute_path()?;

        // Get previous state if any and remove symlinks
        let previous_state = BombadilState::read(absolute_path_to_dot.to_owned());
        let new_state = BombadilState::from(self);

        match previous_state {
            Ok(previous_state) => {
                let diff = previous_state.symlinks.difference(&new_state.symlinks);

                println!("Install diff: {:?}", diff);

                for orphan in diff {
                    if orphan.exists() {
                        if let Ok(canonicalized) = orphan.canonicalize() {
                            unlink(orphan).context(format!(
                                "unlinking `{}`",
                                canonicalized.to_str().to_owned().unwrap().green()
                            ))?;
                            if canonicalized.is_dir() {
                                fs::remove_dir_all(&canonicalized)
                            } else {
                                fs::remove_file(&canonicalized)
                            }
                            .context(format!(
                                "deleting `{}`",
                                canonicalized.to_str().to_owned().unwrap().green()
                            ))?;
                            println!("Deleted - {canonicalized:?} => {orphan:?}");
                        }
                    }
                }
            }
            Err(err) => {
                println!("No previous state: {err}")
            }
        }

        new_state.write()?;

        Ok(())
    }

    /// Unlink dotfiles according to previous state
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

    /// Watch dotfiles and automatically run link on changes
    pub async fn watch(profiles: Vec<String>) -> Result<()> {
        let mut bombadil = Bombadil::from_settings(Mode::Gpg)?;
        bombadil.enable_profiles(profiles.iter().map(String::as_str).collect())?;

        let mut init = InitConfig::default();
        init.on_error(PrintDebug(io::stderr()));

        let dotfiles_path = &bombadil.dotfiles_absolute_path()?;

        let mut runtime = RuntimeConfig::default();
        runtime.action_throttle(Duration::from_secs(1));

        // Ignore stuff like .git dirs
        let ignore_files = ignore_files::from_origin(dotfiles_path).await;
        let ignore_filter = IgnoreFilter::new(dotfiles_path, &ignore_files.0).await?;
        runtime.filterer(Arc::new(IgnoreFilterer(ignore_filter)));

        runtime.pathset([dotfiles_path]);

        runtime.on_action(move |action: Action| {
            let mut b = Bombadil::from_settings(Mode::Gpg).expect("Failed to get settings");
            b.enable_profiles(profiles.iter().map(String::as_str).collect())
                .expect("Failed to enable profiles");

            async move {
                for event in action.events.iter() {
                    // Select only relevant events (creations, modifications, deletions)
                    if event.tags.iter().any(|t| {
                        matches!(t, &Tag::FileEventKind(FileEventKind::Create(_)))
                            || matches!(t, &Tag::FileEventKind(FileEventKind::Modify(_)))
                            || matches!(t, &Tag::FileEventKind(FileEventKind::Remove(_)))
                    }) {
                        println!("{}", "Detected changes, re-linking dots".green());
                        // Finally, install the dots like usual
                        b.install().map_err(|e| RuntimeError::Handler {
                            ctx: "bombadil install",
                            err: e.to_string(),
                        })?;
                        break;
                    }
                }

                let sigs = action
                    .events
                    .iter()
                    .flat_map(|event| event.signals())
                    .collect::<Vec<_>>();

                // Stop gently on Ctrl-C and kill -15
                if sigs
                    .iter()
                    .any(|sig| sig == &MainSignal::Interrupt || sig == &MainSignal::Terminate)
                {
                    action.outcome(Outcome::Exit);
                } else {
                    action.outcome(Outcome::if_running(Outcome::DoNothing, Outcome::Start));
                }

                Ok::<_, RuntimeError>(())
            }
        });

        let watchexec = Watchexec::new(init, runtime.clone())?;
        watchexec.main().await??;
        Ok(())
    }

    /// Add a gpg secret encrypted variable to the target variable file
    pub fn add_secret<S: AsRef<Path> + ?Sized>(
        &self,
        key: &str,
        value: &str,
        var_file: &S,
    ) -> Result<()> {
        if let Some(gpg) = &self.gpg {
            gpg.push_secret(key, value, var_file)
        } else {
            Err(anyhow!("No gpg_user_id in bombadil settings"))
        }
    }

    /// Pretty print current bombadil variables
    pub fn display_vars(&self) {
        self.vars
            .variables
            .iter()
            .for_each(|(key, value)| println!("{} = {}", key.red(), value))
    }

    /// Enable a dotfile profile by merging its settings with the default profile
    pub fn enable_profiles(&mut self, profile_keys: Vec<&str>) -> Result<()> {
        if profile_keys.is_empty() {
            return Ok(());
        }

        self.profile_enabled = profile_keys.iter().map(ToString::to_string).collect();

        let mut profiles: Vec<Profile> = profile_keys
            .iter()
            // unwrap here is safe cause allowed profile keys are checked by clap
            .map(|profile_key| self.profiles.get(&profile_key.to_string()).unwrap())
            .cloned()
            .collect();

        let sub_profiles: Vec<Profile> = profiles
            .iter()
            .flat_map(|profile| {
                profile
                    .extra_profiles
                    .iter()
                    .flat_map(|sub_profile| self.profiles.get(sub_profile))
                    .collect::<Vec<&Profile>>()
            })
            .cloned()
            .collect();

        profiles.extend(sub_profiles);

        // Merge profile dots
        for profile in profiles.iter() {
            profile.dots.iter().for_each(|(key, dot_override)| {
                // Dot exist let's override
                if let Some(dot) = self.dots.get_mut(key) {
                    if let Some(source) = &dot_override.source {
                        dot.source.clone_from(source)
                    }

                    if let Some(target) = &dot_override.target {
                        dot.target.clone_from(target)
                    }

                    if let Some(vars) = &dot_override.vars {
                        dot.vars.clone_from(vars)
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
            let variables = Variables::from_paths(&self.path, &profile.vars)?;
            self.vars.extend(variables);
            // Add Profile pre hooks
            let prehooks = profile
                .prehooks
                .iter()
                .map(|command| command.as_ref())
                .map(|command| {
                    Hook::new(
                        self.path.clone(),
                        command,
                        profile.run_hooks_in_dotfiles_dir,
                    )
                })
                .collect::<Vec<Hook>>();
            self.prehooks.extend(prehooks);

            // Add profile post hooks
            let posthooks = profile
                .posthooks
                .iter()
                .map(|command| command.as_ref())
                .map(|command| {
                    Hook::new(
                        self.path.clone(),
                        command,
                        profile.run_hooks_in_dotfiles_dir,
                    )
                })
                .collect::<Vec<Hook>>();
            self.posthooks.extend(posthooks);
        }

        Ok(())
    }

    fn check_dotfile_dir(&self) -> Result<()> {
        if !self.path.exists() {
            return Err(anyhow!(
                "Dotfiles base path : {}, not found",
                self.path.display(),
            ));
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

    /// Load Bombadil settings from a `bombadil.toml`
    pub fn from_settings(mode: Mode) -> Result<Bombadil> {
        let config = Settings::get()?;
        let path = config.get_dotfiles_path()?;

        let run_hooks_in_dotfiles_dir = config.run_hooks_in_dotfiles_dir();

        let gpg = match mode {
            Mode::Gpg => config.gpg_user_id.map(|user_id| Gpg::new(&user_id)),
            Mode::NoGpg => None,
        };

        // Resolve variables from path
        let mut vars = Variables::from_paths(&path, &config.settings.vars)?;

        // Replace % reference with their ref value
        vars.resolve_ref();

        // Resolve hooks from settings
        let posthooks = config
            .settings
            .posthooks
            .iter()
            .map(|cmd| Hook::new(path.clone(), cmd, run_hooks_in_dotfiles_dir))
            .collect();

        let prehooks = config
            .settings
            .prehooks
            .iter()
            .map(|cmd| Hook::new(path.clone(), cmd, run_hooks_in_dotfiles_dir))
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
            profile_enabled: vec![],
        })
    }

    /// Pretty print metadata, possible values are Dots, PreHooks, PostHook, Path, Profiles, Vars, Secrets
    pub fn print_metadata(
        &self,
        metadata_type: MetadataType,
        writer: &mut impl Write,
    ) -> io::Result<()> {
        let rows = match metadata_type {
            MetadataType::Dots => self
                .dots
                .iter()
                .map(|(k, v)| {
                    format!(
                        "{}: {} => {}",
                        k,
                        self.path.join(&v.source).display(),
                        v.target().unwrap_or_else(|_| v.target.clone()).display()
                    )
                })
                .collect(),
            MetadataType::PreHooks => self.prehooks.iter().map(|h| h.command.clone()).collect(),
            MetadataType::PostHooks => self.posthooks.iter().map(|h| h.command.clone()).collect(),
            MetadataType::Path => vec![self.path.display().to_string()],
            MetadataType::Profiles => {
                let mut profiles = vec!["default".to_string()];
                profiles.extend(self.profiles.keys().cloned());
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
            writer.write_all(rows.join("\n").as_bytes())?;
            writer.flush()?;
        }

        Ok(())
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
            .filter_map(|(_, profile)| profile.dots.get(dot_key))
            .filter(|dot| dot.vars.is_some())
            .filter_map(|dot| dot.resolve_var_path(origin_source))
            .collect();

        let _ = dot_origin.map(|dot| dot.resolve_var_path().map(|path| ignored.push(path)));

        ignored
    }
}

pub enum MetadataType {
    Dots,
    PreHooks,
    PostHooks,
    Path,
    Profiles,
    Vars,
    Secrets,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::unlink;
    use crate::Mode::NoGpg;
    use cmd_lib::{init_builtin_logger, run_cmd};
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;
    use std::ffi::OsStr;
    use std::fs::OpenOptions;
    use std::io::BufWriter;
    use std::{env, fs};

    fn setup(dotfiles: &str) {
        let home_dir = env::current_dir().unwrap().canonicalize().unwrap();
        env::set_var("HOME", home_dir);
        init_builtin_logger();
        run_cmd!(
            mkdir .config;
            tree -a;
        )
        .unwrap();

        Bombadil::link_self_config(Some(PathBuf::from(dotfiles))).unwrap();
    }

    #[sealed_test(files = ["tests/dotfiles_simple"], before = setup("dotfiles_simple"))]
    fn self_link_works() {
        let link = dirs::config_dir().unwrap().join(BOMBADIL_CONFIG);

        assert_that!(link).exists();
    }

    #[sealed_test(files = ["tests/dotfiles_simple"], before = setup("dotfiles_simple"))]
    fn install_single_file_works() -> Result<()> {
        Bombadil::from_settings(NoGpg)?.install()?;

        let target = fs::read_link(".config/template.css")?;
        let expected = env::current_dir()?.join("dotfiles_simple/.dots/template.css");

        assert_that!(target).is_equal_to(expected);

        let target = std::fs::read_to_string(target)?;

        assert_eq!(
            target,
            indoc! {
                ".class {
                    color: #de1f1f
                }
                "
            }
        );

        Ok(())
    }

    #[sealed_test(files = ["tests/dotfiles_create_dir"], before = setup("dotfiles_create_dir"))]
    fn install_creates_missing_directories() -> Result<()> {
        Bombadil::from_settings(NoGpg)?.install()?;

        let link = env::current_dir()?.join(".config/sub/dir/template.css");
        let target = std::fs::read_to_string(link)?;

        assert_eq!(
            target,
            indoc! {
                ".class {
                    color: #de1f1f
                }
                "
            }
        );

        Ok(())
    }

    #[sealed_test(files = ["tests/dotfiles_invalid_dot"], before = setup("dotfiles_invalid_dot"))]
    fn install_should_fail_and_continue() -> Result<()> {
        // Act
        Bombadil::from_settings(NoGpg)?.install()?;
        run_cmd!(tree -a;)?;
        // Assert
        assert_that!(PathBuf::from(".config/template.css")).exists();
        assert_that!(PathBuf::from(".config/invalid")).does_not_exist();
        Ok(())
    }

    #[sealed_test(files = ["tests/dotfiles_simple"], before = setup("dotfiles_simple"))]
    fn uninstall_works() -> Result<()> {
        Bombadil::link_self_config(Some(PathBuf::from("dotfiles_simple")))?;
        let bombadil = Bombadil::from_settings(NoGpg)?;

        bombadil.install()?;
        assert_that!(PathBuf::from(".config/template.css")).exists();

        bombadil.uninstall()?;
        assert_that!(PathBuf::from(".config/template.css")).does_not_exist();
        Ok(())
    }

    #[sealed_test(files = ["tests/dotfiles_simple"], before = setup("dotfiles_simple"))]
    fn posthook_ok() -> Result<()> {
        let bombadil = Bombadil::from_settings(NoGpg)?;

        // Act
        bombadil.install()?;

        // Assert
        assert_that!(PathBuf::from(".config/posthook/file").exists());

        Ok(())
    }

    #[sealed_test(files = ["tests/dotfiles_simple"], before = setup("dotfiles_simple"))]
    fn prehook_ok() -> Result<()> {
        let bombadil = Bombadil::from_settings(NoGpg)?;

        // Act
        bombadil.install()?;

        // Assert
        assert_that!(PathBuf::from(".config/prehook_file")).exists();

        Ok(())
    }

    #[sealed_test(files = ["tests/dotfiles_with_meta"], before = setup("dotfiles_with_meta"))]
    fn meta_var_works() -> Result<()> {
        // Act
        let bombadil = Bombadil::from_settings(NoGpg)?;

        // Assert
        assert_that!(bombadil.vars.variables.get("red"))
            .is_some()
            .is_equal_to(&"#FF0000".to_string());

        assert_that!(bombadil.vars.variables.get("black"))
            .is_some()
            .is_equal_to(&"#000000".to_string());

        assert_that!(bombadil.vars.variables.get("green"))
            .is_some()
            .is_equal_to(&"#008000".to_string());

        Ok(())
    }

    #[sealed_test(files = [ "tests/dotfiles_with_meta" ], before = setup("dotfiles_with_meta"))]
    fn should_print_metadata() -> Result<()> {
        let bombadil = Bombadil::from_settings(NoGpg)?;

        let mut content = vec![];
        let mut writer = BufWriter::new(&mut content);

        // Act
        bombadil.print_metadata(MetadataType::Vars, &mut writer)?;
        let result = String::from_utf8(writer.get_ref().to_vec())?;
        let result = result.as_str();

        // Assert
        assert_that!(result).contains("black: #000000");
        assert_that!(result).contains("green: #008000");
        assert_that!(result).contains("red: #FF0000");
        assert_that!(result).contains("meta_red: #FF0000");

        Ok(())
    }

    #[sealed_test(files = [ "tests/dotfiles_with_nested_dir" ], before = setup("dotfiles_with_nested_dir"))]
    fn should_get_auto_ignored_files() -> Result<()> {
        let bombadil = Bombadil::from_settings(NoGpg)?;

        let ignored_files = bombadil.get_auto_ignored_files("sub_dir");
        let ignored_files: Vec<&str> = ignored_files
            .iter()
            .filter_map(|path| path.file_name())
            .filter_map(OsStr::to_str)
            .collect();

        assert_that!(ignored_files).contains("vars.toml");

        Ok(())
    }

    #[sealed_test]
    fn should_unlink_dir() -> Result<()> {
        run_cmd!(
            mkdir "directory";
            ln -sf "directory" "linked_directory";
        )?;

        unlink("linked_directory")?;

        assert_that!(PathBuf::from("directory")).exists();
        assert_that!(PathBuf::from("linked_directory")).does_not_exist();

        Ok(())
    }

    #[sealed_test]
    fn should_unlink_file() -> Result<()> {
        run_cmd!(
            echo "Hello Tom" > "file";
            ln -sf file link;
        )?;

        unlink("link")?;

        assert_that!(PathBuf::from("file")).exists();
        assert_that!(PathBuf::from("link")).does_not_exist();

        Ok(())
    }

    #[sealed_test(files = ["tests/dot_files_with_imports"], before = setup("dot_files_with_imports"))]
    fn should_merge_import() -> Result<()> {
        // Arrange
        let bombadil = Bombadil::from_settings(NoGpg)?;

        assert_that!(bombadil.dots.get("maven")).is_some();
        assert_that!(bombadil.vars.variables.get("hello"))
            .is_some()
            .is_equal_to(&"world".to_string());

        assert_that!(bombadil.dots.get("relative_import/maven_relative")).is_some();
        assert_that!(
            bombadil
                .dots
                .get("relative_import/maven_relative")
                .unwrap()
                .source
        )
        .is_equal_to(PathBuf::from("relative_import/settings.xml"));
        Ok(())
    }

    #[sealed_test(files = ["tests/dotfiles_with_profile_context"], before = setup("dotfiles_with_profile_context"))]
    fn should_have_profile_context() -> Result<()> {
        // Arrange
        let mut bombadil = Bombadil::from_settings(NoGpg)?;
        bombadil.enable_profiles(vec!["fancy"])?;

        // Act
        bombadil.install()?;
        let target = fs::read_link(".config/template.css")?;
        let content = fs::read_to_string(target)?;

        // Assert
        assert_that!(content).is_equal_to(".class {color: #de1f1f}\n".to_string());

        Ok(())
    }

    #[sealed_test(files = ["tests/dotfiles_backup"], before = setup("dotfiles_backup"))]
    fn should_move_existing_file_to_backup() -> Result<()> {
        let bombadil = Bombadil::from_settings(NoGpg)?;

        let original_path = PathBuf::from(".config/deep/test/dir/template.css");
        assert_that!(original_path).does_not_exist();

        fs::create_dir_all(".config/deep/test/dir")?;
        OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(original_path.clone())?;

        let expected_backup_path = env::current_dir()?.join(format!(
            "dotfiles_backup/.backups{}/.config/deep/test/dir/template.css",
            env::current_dir()?.display()
        ));

        assert_that!(expected_backup_path).does_not_exist();

        bombadil.install()?;

        assert_that!(expected_backup_path).exists();

        let target = std::fs::read_to_string(original_path)?;
        assert_eq!(
            target,
            indoc! {
                ".class {
                    color: #de1f1f
                }
                "
            }
        );

        Ok(())
    }
}
