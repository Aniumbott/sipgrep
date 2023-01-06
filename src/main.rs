use sipgrep::Config;
use std::{env, process};
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|_err| {
        eprintln!("Problem parsing arguments: {_err}");
        process::exit(1);
    });

    if let Err(_e) = sipgrep::run(config) {
        eprintln!("Application error: {_e}");
        process::exit(1);
    }
}
