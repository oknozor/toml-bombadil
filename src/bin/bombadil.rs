use clap::{App, Arg, SubCommand};
use toml_bombadil::{edit_links, load_theme, install, list_themes};

const LINK: &str = "link";
const THEME: &str = "theme";
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
        .subcommand(
            SubCommand::with_name(THEME)
                .help("symlink your dotfiles according to bombadil.toml config")
                .arg(Arg::with_name("set")
                    .help("set the current theme and update your dotfiles accordingly")
                    .short("ls")
                    .long("set")
                    .takes_value(true)
                    .required_unless_one(&["list", "load"])
                )
                .arg(
                    Arg::with_name("list")
                        .short("l")
                        .long("list")
                        .help("list available theme")
                        .required_unless_one(&["set", "load"]),
                )
                .arg(
                    Arg::with_name("load")
                        .short("l")
                        .long("load")
                        .help("load current theme")
                        .required_unless_one(&["set", "list"]),
                )
        )
        .get_matches();

    if let Some(subcommand) = matches.subcommand_name() {
        match subcommand {
            INSTALL => {
                let install_commmand = matches
                    .subcommand_matches(INSTALL)
                    .unwrap();

                let config_path = install_commmand
                    .value_of("CONFIG")
                    .unwrap();

                let _command_result = install(config_path)
                    .unwrap();
            }

            LINK => {
                let _command_result = edit_links();
            }

            THEME => {
                let theme_command = matches
                    .subcommand_matches(THEME)
                    .unwrap();

                if theme_command.is_present("list") {
                    list_themes()
                        .unwrap_or_else(|err| fatal!("{}", err))
                        .iter()
                        .for_each(|path| println!("{}", path.display()));

                } else if theme_command.is_present("set") {
                    println!("{}", theme_command.value_of("set").unwrap());
                    todo!("set theme")
                } else if theme_command.is_present("load") {
                    load_theme().unwrap()
                }
            }

            _ => unreachable!()
        }
    }
}
