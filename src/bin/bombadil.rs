use clap::{App, AppSettings, Arg, SubCommand};
use colored::Colorize;
use std::path::Path;
use toml_bombadil::dots::Dot;
use toml_bombadil::settings::Settings;
use toml_bombadil::Bombadil;

const LINK: &str = "link";
const INSTALL: &str = "install";

macro_rules! fatal {
    ($($tt:tt)*) => {{
        use std::io::Write;
        writeln!(&mut ::std::io::stderr(), $($tt)*).unwrap();
        ::std::process::exit(1)
    }}
}

fn main() {
    let dots = Settings::get()
        .map(|settings| settings.dot)
        .unwrap_or_default();

    let subcommand_settings = &[
        AppSettings::UnifiedHelpMessage,
        AppSettings::ColoredHelp,
        AppSettings::VersionlessSubcommands,
    ];
    let app_settings = &[
        AppSettings::SubcommandRequiredElseHelp,
        AppSettings::UnifiedHelpMessage,
        AppSettings::ColoredHelp,
        AppSettings::VersionlessSubcommands,
    ];

    let unnamed_dots_with_profile: Vec<&Dot> = dots
        .iter()
        .filter(|dot| dot.name.is_none() && dot.profile.is_some())
        .collect();

    let profile_subcommands = dots
        .iter()
        .filter(|dot| dot.profile.is_some() && dot.name.is_some())
        .map(|dot| {
            let mut profiles = dot.get_profile_names().to_vec();
            profiles.push("default");

            SubCommand::with_name(&dot.name.as_ref().unwrap())
                .settings(subcommand_settings)
                .about("User defined profile command")
                .arg(
                    Arg::with_name("PROFILE")
                        .required(true)
                        .long("set-profile")
                        .short("s")
                        .possible_values(profiles.as_slice())
                        .takes_value(true)
                        .help("Switch to a valid profile defined in your bombadil config"),
                )
        })
        .collect::<Vec<App>>();

    let matches = App::new("Toml Bombadil")
        .settings(app_settings)
        .version(env!("CARGO_PKG_VERSION"))
        .author("Paul D. <paul.delafosse@protonmail.com>")
        .about("A dotfile template manager")
        .long_about("Toml is a dotfile template manager, written in rust. \
        For more info on how to configure it please go to https://github.com/oknozor/toml-bombadil")
        .subcommand(
            SubCommand::with_name(INSTALL)
                .about("Link a given bombadil config to XDG_CONFIG_DIR/bombadil.toml")
                .arg(Arg::with_name("CONFIG")
                    .help("path to your bombadil.toml config file inside your dotfiles directory")
                    .short("c")
                    .long("config")
                    .takes_value(true)
                    .required(true)),
        )
        .subcommand(
            SubCommand::with_name(LINK)
                .about("Symlink a copy of your dotfiles  and inject variables according to bombadil.toml config"),
        )
        .subcommands(profile_subcommands)
        .get_matches();

    if let Some(subcommand) = matches.subcommand_name() {
        match subcommand {
            INSTALL => {
                let install_commmand = matches.subcommand_matches(INSTALL).unwrap();
                let config_path = install_commmand
                    .value_of("CONFIG")
                    .map(|config_path| Path::new(config_path).to_path_buf());

                Bombadil::link_self_config(config_path).unwrap_or_else(|err| fatal!("{}", err));
            }

            LINK => {
                let bombadil = Bombadil::from_settings().unwrap_or_else(|err| fatal!("{}", err));

                let _command_result = bombadil.install().unwrap_or_else(|err| fatal!("{}", err));
            }

            named_dot => {
                let mut bombadil =
                    Bombadil::from_settings().unwrap_or_else(|err| fatal!("{}", err));
                let profile_command = matches.subcommand_matches(named_dot).unwrap();

                let profile_name = profile_command.value_of("PROFILE").unwrap();

                bombadil
                    .update_profile(named_dot, profile_name)
                    .unwrap_or_else(|err| fatal!("{:?}", err));
            }
        }
    }

    if !unnamed_dots_with_profile.is_empty() {
        unnamed_dots_with_profile.iter().for_each(|dot| {
            println!(
                "{} {:?}",
                "Some profile are define an unnamed dot entry :".yellow(),
                &dot.source
            )
        });
    }
}
