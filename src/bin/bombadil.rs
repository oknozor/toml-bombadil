use clap::{App, AppSettings, Arg, SubCommand};
use std::path::PathBuf;
use toml_bombadil::settings::Settings;
use toml_bombadil::Bombadil;

const LINK: &str = "link";
const INSTALL: &str = "install";
const ADD_SECRET: &str = "add-secret";
const REMOVE_SECRET: &str = "remove-secret";
const SHOW_SECRETS: &str = "show-secrets";

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
                .required(true))
        )
        .subcommand(SubCommand::with_name(REMOVE_SECRET)
            .settings(subcommand_settings)
            .about("Remove a secret var from bombadil environment")
            .arg(Arg::with_name("key")
                .help("Key of the secret variable to remove")
                .short("k")
                .long("key")
                .takes_value(true)
                .required(true))
        )
        .subcommand(SubCommand::with_name(SHOW_SECRETS)
            .settings(subcommand_settings)
            .about("Show currently stored secret vars")
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
                    Bombadil::from_settings().unwrap_or_else(|err| fatal!("{}", err));
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
            ADD_SECRET => {
                let add_secret_subcommand = matches.subcommand_matches(ADD_SECRET).unwrap();
                let key = add_secret_subcommand.value_of("key").unwrap();
                let value = add_secret_subcommand.value_of("value").unwrap();

                let bombadil = Bombadil::from_settings().unwrap_or_else(|err| fatal!("{}", err));

                bombadil
                    .add_secret(key, value)
                    .unwrap_or_else(|err| fatal!("{}", err));
            }
            REMOVE_SECRET => {
                let add_secret_subcommand = matches.subcommand_matches(REMOVE_SECRET).unwrap();
                let key = add_secret_subcommand.value_of("key").unwrap();

                let bombadil = Bombadil::from_settings().unwrap_or_else(|err| fatal!("{}", err));

                bombadil
                    .remove_secret(key)
                    .unwrap_or_else(|err| fatal!("{}", err));
            }
            SHOW_SECRETS => {
                let bombadil = Bombadil::from_settings().unwrap_or_else(|err| fatal!("{}", err));
                bombadil.show_secrets()
                    .unwrap_or_else(|err| fatal!("{}", err));
            }

            _ => unreachable!(),
        }
    }
}
