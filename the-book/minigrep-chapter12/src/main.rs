use std::env;
use std::process;

use::minigrep::Config;

fn main() {

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "the query you gave is {}\nthe file_path you gace is {}\n\n",
        config.query, config.file_path
    );

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
