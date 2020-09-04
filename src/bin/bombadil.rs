use clap::{App, AppSettings, Arg, SubCommand};
use std::path::PathBuf;
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
    let app_settings = &[
        AppSettings::SubcommandRequiredElseHelp,
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
        .get_matches();

    if let Some(subcommand) = matches.subcommand_name() {
        match subcommand {
            INSTALL => {
                let install_commmand = matches.subcommand_matches(INSTALL).unwrap();
                let config_path = install_commmand.value_of("CONFIG").map(PathBuf::from);

                Bombadil::link_self_config(config_path).unwrap_or_else(|err| fatal!("{}", err));
            }

            LINK => {
                let bombadil = Bombadil::from_settings().unwrap_or_else(|err| fatal!("{}", err));

                let _command_result = bombadil.install().unwrap_or_else(|err| fatal!("{}", err));
            }
            _ => unreachable!(),
        }
    }
}
