use clap::{App, Arg, SubCommand};
use std::path::Path;
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
    let matches = App::new("Toml Bombadil")
        .version("0.1")
        .author("Paul D. <paul.delafosse@protonmail.com>")
        .about("A dotfile and theme manager")
        .subcommand(
            SubCommand::with_name(INSTALL).arg(
                Arg::with_name("CONFIG")
                    .help("path to your bombadil.toml config file inside your dotfiles directory")
                    .short("c")
                    .long("config")
                    .takes_value(true)
                    .required(true),
            ),
        )
        .subcommand(
            SubCommand::with_name(LINK)
                .help("symlink your dotfiles according to bombadil.toml config"),
        )
        .get_matches();

    if let Some(subcommand) = matches.subcommand_name() {
        match subcommand {
            INSTALL => {
                let install_commmand = matches.subcommand_matches(INSTALL).unwrap();
                let config_path = install_commmand
                    .value_of("CONFIG")
                    .map(|config_path| Path::new(config_path).to_path_buf());
                let bombadil = Bombadil::from_settings().unwrap();

                bombadil
                    .link_self_config(config_path)
                    .unwrap_or_else(|err| fatal!("{}", err));
            }

            LINK => {
                let bombadil = Bombadil::from_settings().unwrap_or_else(|err| fatal!("{}", err));

                let _command_result = bombadil.install().unwrap_or_else(|err| fatal!("{}", err));
            }

            _ => unreachable!(),
        }
    }
}
