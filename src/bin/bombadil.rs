use clap::lazy_static::lazy_static;
use clap::{AppSettings, IntoApp, Parser};
use clap_complete::Shell;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use toml_bombadil::settings::Settings;
use toml_bombadil::{Bombadil, MetadataType, Mode};

macro_rules! fatal {
    ($($tt:tt)*) => {{
        use std::io::Write;
        writeln!(&mut ::std::io::stderr(), $($tt)*).unwrap();
        ::std::process::exit(1)
    }}
}

lazy_static! {
    static ref SETTINGS: Settings = Settings::get().unwrap_or_default();
}

fn profiles() -> Vec<&'static str> {
    SETTINGS
        .profiles
        .keys()
        .map(|profile| profile.as_ref())
        .collect()
}

/// Toml is a dotfile template manager, written in rust.
#[derive(Parser, Debug)]
#[clap(
    global_setting = AppSettings::DeriveDisplayOrder,
    subcommand_required = true,
    arg_required_else_help = true,
    version,
    name = "Toml Bombadil",
    author = "Paul D. <paul.delafosse@protonmail.com>"
)]
enum Cli {
    /// Link a given dotfile directory config to "XDG_CONFIG_DIR/bombadil.toml"
    Install {
        /// Path to your dotfile directory
        #[clap(value_name = "CONFIG", required = false)]
        config: Option<PathBuf>,
    },
    /// Install dotfiles from a remote git repository to a target folder
    Clone {
        /// Remote repository address, either http or ssh
        #[clap(short, long, required = false)]
        remote: String,
        /// Target destination, repository name by default
        #[clap(short, long, required = false)]
        target: Option<PathBuf>,
        /// A list of comma separated profiles to activate
        #[clap(short, long, required = false, multiple_values = true)]
        profiles: Vec<String>,
    },
    /// Symlink a copy of your dotfiles and inject variables according to bombadil.toml config
    Link {
        /// A list of comma separated profiles to activate
        #[clap(short, long, required = false, multiple_values = true, possible_values = profiles())]
        profiles: Vec<String>,
    },
    /// Remove all symlinks defined in your bombadil.toml
    Unlink,
    /// Add a secret var to bombadil environment
    AddSecret {
        /// Key of the secret variable to create
        #[clap(short, long)]
        key: String,
        #[clap(short, long, required_unless_present = "ask")]
        value: String,
        /// Get the secret value from stdin
        #[clap(long, short)]
        ask: bool,
        /// Path of the var file to modify
        #[clap(long, short)]
        file: String,
    },
    /// Get metadata about dots, hooks, path, profiles, or vars
    Get {
        #[clap(value_name = "VALUE", possible_values = &["dots", "prehooks", "posthooks", "path", "profiles", "vars", "secrets"])]
        value: String,
        #[clap(multiple_values = true, possible_values = profiles())]
        profiles: Vec<String>,
    },
    /// Generate shell completions
    /// Generate shell completions
    GenerateCompletions {
        /// Type of completions to generate
        #[clap(name = "type", arg_enum)]
        shell: Shell,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli {
        Cli::Install { config } => {
            Bombadil::link_self_config(config).unwrap_or_else(|err| fatal!("{}", err));
        }
        Cli::Clone {
            remote,
            target,
            profiles,
        } => {
            let path = match target {
                None => {
                    let repo_name = remote.split('/').last().unwrap();
                    let repo_name = repo_name.strip_suffix(".git").unwrap();
                    PathBuf::from_str(repo_name).unwrap()
                }
                Some(path) => path,
            };

            println!("Cloning {remote} in {path:?}");
            let profiles: Option<Vec<&str>> = if !profiles.is_empty() {
                // Remove this
                let vec = profiles.iter().map(String::as_str).collect();
                Some(vec)
            } else {
                None
            };

            Bombadil::install_from_remote(&remote, path, profiles)
                .unwrap_or_else(|err| fatal!("{}", err));
        }
        Cli::Link { profiles } => {
            let mut bombadil =
                Bombadil::from_settings(Mode::Gpg).unwrap_or_else(|err| fatal!("{}", err));

            if !profiles.is_empty() {
                bombadil
                    .enable_profiles(profiles.iter().map(String::as_str).collect())
                    .unwrap_or_else(|err| fatal!("{}", err));
            }

            bombadil.install().unwrap_or_else(|err| fatal!("{}", err));
        }
        Cli::Unlink => {
            Bombadil::from_settings(Mode::NoGpg)
                .and_then(|bombadil| bombadil.uninstall())
                .unwrap_or_else(|err| fatal!("{}", err));
        }
        Cli::AddSecret {
            key,
            value,
            ask,
            file,
        } => {
            let value = if ask {
                println!("Type the value and press enter to confirm :");
                std::io::stdin().lock().lines().next().unwrap().unwrap()
            } else {
                value
            };

            let var_file = file;
            let path = Path::new(&var_file);

            if !path.exists() {
                fatal!(
                    "Error trying to write secret to {} : No such file",
                    var_file
                )
            };

            if path.is_dir() {
                fatal!(
                    "Error trying to write secret to {} : is a directory",
                    var_file
                )
            }

            Bombadil::from_settings(Mode::Gpg)
                .and_then(|bombadil| bombadil.add_secret(&key, &value, &var_file))
                .unwrap_or_else(|err| fatal!("{}", err));
        }
        Cli::Get { value, profiles } => {
            let metadata_type = match value.as_str() {
                "dots" => MetadataType::Dots,
                "prehooks" => MetadataType::PreHooks,
                "posthooks" => MetadataType::PostHooks,
                "path" => MetadataType::Path,
                "profiles" => MetadataType::Profiles,
                "vars" => MetadataType::Vars,
                "secrets" => MetadataType::Secrets,
                _ => unreachable!(),
            };

            let mut bombadil = match metadata_type {
                MetadataType::Secrets => Bombadil::from_settings(Mode::Gpg),
                _ => Bombadil::from_settings(Mode::NoGpg),
            }
            .unwrap_or_else(|err| fatal!("{}", err));

            if !profiles.is_empty() {
                bombadil
                    .enable_profiles(profiles.iter().map(String::as_str).collect())
                    .unwrap_or_else(|err| fatal!("{}", err));
            }

            bombadil.print_metadata(metadata_type);
        }
        Cli::GenerateCompletions { shell } => {
            clap_complete::generate(
                shell,
                &mut Cli::command(),
                "bombadil",
                &mut std::io::stdout(),
            );
        }
    };
}
