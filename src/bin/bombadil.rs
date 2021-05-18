use clap::Shell;
use std::io::BufRead;
use std::path::PathBuf;
use toml_bombadil::cli;
use toml_bombadil::settings::Settings;
use toml_bombadil::{Bombadil, MetadataType, Mode};

macro_rules! fatal {
    ($($tt:tt)*) => {{
        use std::io::Write;
        writeln!(&mut ::std::io::stderr(), $($tt)*).unwrap();
        ::std::process::exit(1)
    }}
}

fn main() {
    let profiles = Settings::get()
        .map(|settings| settings.profiles)
        .unwrap_or_default();

    let profile_names = profiles
        .iter()
        .map(|profile| profile.0.as_str())
        .collect::<Vec<&str>>();

    let matches = cli::build_cli(profile_names.clone()).get_matches();

    if let Some(subcommand) = matches.subcommand_name() {
        match subcommand {
            cli::INSTALL => {
                let install_commmand = matches.subcommand_matches(cli::INSTALL).unwrap();
                let config_path = install_commmand.value_of("CONFIG").map(PathBuf::from);

                Bombadil::link_self_config(config_path).unwrap_or_else(|err| fatal!("{}", err));
            }

            cli::LINK => {
                let mut bombadil =
                    Bombadil::from_settings(Mode::Gpg).unwrap_or_else(|err| fatal!("{}", err));
                let link_command = matches.subcommand_matches(cli::LINK).unwrap();

                if link_command.is_present("profiles") {
                    let profiles: Vec<_> = link_command.values_of("profiles").unwrap().collect();
                    let _command_result = bombadil
                        .enable_profiles(profiles)
                        .unwrap_or_else(|err| fatal!("{}", err));
                }

                bombadil.install().unwrap_or_else(|err| fatal!("{}", err));
            }
            cli::UNLINK => {
                let bombadil =
                    Bombadil::from_settings(Mode::NoGpg).unwrap_or_else(|err| fatal!("{}", err));
                bombadil.uninstall().unwrap_or_else(|err| fatal!("{}", err));
            }
            cli::ADD_SECRET => {
                let add_secret_subcommand = matches.subcommand_matches(cli::ADD_SECRET).unwrap();
                let key = add_secret_subcommand.value_of("key").unwrap();

                let value = if add_secret_subcommand.is_present("ask") {
                    println!("Type the value and press enter to confirm :");
                    std::io::stdin().lock().lines().next().unwrap().unwrap()
                } else {
                    add_secret_subcommand.value_of("value").unwrap().to_string()
                };

                let var_file = add_secret_subcommand.value_of("file").unwrap();

                let bombadil =
                    Bombadil::from_settings(Mode::Gpg).unwrap_or_else(|err| fatal!("{}", err));

                bombadil
                    .add_secret(key, &value, var_file)
                    .unwrap_or_else(|err| fatal!("{}", err));
            }
            cli::GET => {
                let get_subcommand = matches.subcommand_matches(cli::GET).unwrap();
                let metadata_type = match get_subcommand.value_of("value").unwrap() {
                    "dots" => MetadataType::Dots,
                    "hooks" => MetadataType::Hooks,
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

                if get_subcommand.is_present("profiles") {
                    let profiles: Vec<_> = get_subcommand.values_of("profiles").unwrap().collect();
                    let _command_result = bombadil
                        .enable_profiles(profiles)
                        .unwrap_or_else(|err| fatal!("{}", err));
                }

                bombadil.print_metadata(metadata_type);
            }
            cli::GENERATE_COMPLETIONS => {
                let generate_subcommand = matches
                    .subcommand_matches(cli::GENERATE_COMPLETIONS)
                    .unwrap();
                let for_shell = match generate_subcommand.value_of("type").unwrap() {
                    "bash" => Shell::Bash,
                    "elvish" => Shell::Elvish,
                    "fish" => Shell::Fish,
                    "zsh" => Shell::Zsh,
                    _ => unreachable!(),
                };
                cli::build_cli(profile_names).gen_completions_to(
                    "bombadil",
                    for_shell,
                    &mut std::io::stdout(),
                );
            }
            _ => unreachable!(),
        }
    }
}
