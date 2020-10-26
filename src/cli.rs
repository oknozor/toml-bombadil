use clap::{App, AppSettings, Arg, SubCommand};

pub const LINK: &str = "link";
pub const UNLINK: &str = "unlink";
pub const INSTALL: &str = "install";
pub const ADD_SECRET: &str = "add-secret";
pub const GET: &str = "get";
pub const GENERATE_COMPLETIONS: &str = "generate-completions";

pub fn build_cli<'a, 'b>(profile_names: Vec<&'a str>) -> App<'a, 'b>
where
    'a: 'b,
{
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

    App::new("Toml Bombadil")
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
            .arg(Arg::with_name("profiles")
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
            .arg(Arg::with_name("profiles")
                .short("p")
                .long("profiles")
                .takes_value(true)
                .possible_values(profile_names.as_slice())
                .multiple(true)
                .help("Get metadata for specific profiles")
            )
        )
        .subcommand(SubCommand::with_name(GENERATE_COMPLETIONS)
            .settings(subcommand_settings)
            .about("Generate shell completions")
            .arg(Arg::with_name("type")
                .possible_values(&["bash", "elvish", "fish", "zsh"])
                .required(true)
                .takes_value(true)
                .help("Type of completions to generate")
            )
        )
}
