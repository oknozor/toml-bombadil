extern crate clap;

use std::env;

use clap::Shell;

include!("src/cli.rs");

fn main() {
    let outdir = match env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };
    let mut app = build_cli(vec![]);
    vec![Shell::Bash, Shell::Elvish, Shell::Fish, Shell::Zsh]
        .iter()
        .for_each(|shell| {
            app.gen_completions("bombadil", *shell, outdir.clone());
        });
}
