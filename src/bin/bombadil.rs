use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Toml Bombadil")
        .version("0.1")
        .author("Paul D. <paul.delafosse@protonmail.com>")
        .about("A dotfile and theme manager")
        .subcommand(
            SubCommand::with_name("install").arg(
                Arg::with_name("CONFIG")
                    .short("c")
                    .long("config")
                    .help("path to your bombadil.toml config file inside your dotfiles directory")
                    .required(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("link")
                .help("symlink your dotfiles according to bombadil.toml config"),
        )
        .subcommand(
            SubCommand::with_name("theme")
                .help("symlink your dotfiles according to bombadil.toml config"),
        )
        .get_matches();
}
