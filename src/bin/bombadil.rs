use clap::{App, AppSettings, Arg, SubCommand};
use std::io::BufRead;
use std::path::PathBuf;
use toml_bombadil::settings::Settings;
use toml_bombadil::{Bombadil, MetadataType, Mode};

const LINK: &str = "link";
const UNLINK: &str = "unlink";
const INSTALL: &str = "install";
const ADD_SECRET: &str = "add-secret";
const GET: &str = "get";

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

    let app_settings = &[
        AppSettings::SubcommandRequiredElseHelp,
        AppSettings::UnifiedHelpMessage,
        AppSettings::ColoredHelp,
        AppSettings::VersionlessSubcommands,
    ];

    let subcommand_settings = &[
        AppSettings::UnifiedHelpMessage,
        AppSettings::ColoredHelp,
        AppSettings::VersionlessSubcommands,
    ];

    let matches = App::new("Toml Bombadil")
        .settings(app_settings)
        .version(env!("CARGO_PKG_VERSION"))
        .author("Paul D. <paul.delafosse@protonmail.com>")
        .about("A dotfile template manager")
        .long_about("Toml is a dotfile template manager, written in rust. \
        For more info on how to configure it please go to https://github.com/oknozor/toml-bombadil")
        .subcommand(SubCommand::with_name(INSTALL)
            .settings(subcommand_settings)
            .about("Link a given bombadil config to XDG_CONFIG_DIR/bombadil.toml")
            .arg(Arg::with_name("CONFIG")
                .help("path to your bombadil.toml config file inside your dotfiles directory")
                .short("c")
                .long("config")
                .takes_value(true)
                .required(true)))
        .subcommand(SubCommand::with_name(LINK)
            .settings(subcommand_settings)
            .about("Symlink a copy of your dotfiles and inject variables according to bombadil.toml config")
            .arg(Arg::with_name("PROFILES")
                .help("A list of comma separated profiles to activate")
                .short("p")
                .long("profiles")
                .possible_values(profile_names.as_slice())
                .takes_value(true)
                .multiple(true)
                .required(false)))
        .subcommand(SubCommand::with_name(UNLINK)
            .settings(subcommand_settings)
            .about("Remove all symlinks defined in your bombadil.toml"))
        .subcommand(SubCommand::with_name(ADD_SECRET)
            .settings(subcommand_settings)
            .about("Add a secret var to bombadil environment")
            .arg(Arg::with_name("key")
                .help("Key of the secret variable to create")
                .short("k")
                .long("key")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("value")
                .help("Value of the secret variable to create")
                .short("v")
                .long("value")
                .takes_value(true)
                .required_unless("ask"))
            .arg(Arg::with_name("ask")
                .help("Get the secret value from stdin")
                .short("a")
                .long("ask")
                .takes_value(false)
                .required_unless("value"))
            .arg(Arg::with_name("file")
                .help("Path of the var file to modify")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(true)))
        .subcommand(SubCommand::with_name(GET)
            .settings(subcommand_settings)
            .about("Get metadata about dots, hooks, path, profiles, or vars")
            .arg(Arg::with_name("value")
                .possible_values(&["dots", "hooks", "path", "profiles", "vars", "secrets"])
                .default_value("dots")
                .takes_value(true)
                .help("Metadata to get"))
        )
        .get_matches();

    if let Some(subcommand) = matches.subcommand_name() {
        match subcommand {
            INSTALL => {
                let install_commmand = matches.subcommand_matches(INSTALL).unwrap();
                let config_path = install_commmand.value_of("CONFIG").map(PathBuf::from);

                Bombadil::link_self_config(config_path).unwrap_or_else(|err| fatal!("{}", err));
            }

            LINK => {
                let mut bombadil =
                    Bombadil::from_settings(Mode::Gpg).unwrap_or_else(|err| fatal!("{}", err));
                let link_command = matches.subcommand_matches(LINK).unwrap();

                if link_command.is_present("PROFILES") {
                    let profiles: Vec<_> = link_command.values_of("PROFILES").unwrap().collect();
                    let _command_result = bombadil
                        .enable_profiles(profiles)
                        .unwrap_or_else(|err| fatal!("{}", err));
                } else {
                    let _command_result =
                        bombadil.install().unwrap_or_else(|err| fatal!("{}", err));
                }
            }
            UNLINK => {
                let bombadil =
                    Bombadil::from_settings(Mode::NoGpg).unwrap_or_else(|err| fatal!("{}", err));
                bombadil.uninstall().unwrap_or_else(|err| fatal!("{}", err));
            }
            ADD_SECRET => {
                let add_secret_subcommand = matches.subcommand_matches(ADD_SECRET).unwrap();
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
            GET => {
                let get_subcommand = matches.subcommand_matches(GET).unwrap();
                let metadata_type = match get_subcommand.value_of("value").unwrap() {
                    "dots" => MetadataType::Dots,
                    "hooks" => MetadataType::Hooks,
                    "path" => MetadataType::Path,
                    "profiles" => MetadataType::Profiles,
                    "vars" => MetadataType::Vars,
                    "secrets" => MetadataType::Secrets,
                    _ => unreachable!(),
                };

                let bombadil = match metadata_type {
                    MetadataType::Secrets => Bombadil::from_settings(Mode::Gpg),
                    _ => Bombadil::from_settings(Mode::NoGpg),
                }
                .unwrap_or_else(|err| fatal!("{}", err));

                bombadil.print_metadata(metadata_type);
            }

            _ => unreachable!(),
        }
    }
}
