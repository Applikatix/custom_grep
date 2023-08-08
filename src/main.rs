use std::{env, process};
use custom_grep::Config;

fn main() {
    let config =
        Config::from(env::args())
        .unwrap_or_else(|err| {
            eprintln!("Couldn't parse arguments: {err}");
            process::exit(1);
        });

    if let Err(err) = custom_grep::run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}
