use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Parsing arguments failed: {err}");
        process::exit(1);
    });
    println!(
        "Searching for {} in file {}",
        config.query, config.file_path
    );
    if let Err(e) = minigrep::run(config) {
        eprintln!("Minigrep failed : {e}");
        process::exit(1);
    }
}
