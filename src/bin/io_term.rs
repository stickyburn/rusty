use std::{env, process};

use rusty::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error setting arguments: {err}");
        process::exit(1);
    });

    if let Err(_) = rusty::run(config) {
        process::exit(1);
    }
}
