extern crate microgrep;

use std::env;
use std::io::{self};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("microgrep takes one argument");
    }
    let query = &args[1];

    let stdin = io::stdin();
    let stdout = io::stdout();

    if let Err(e) = microgrep::filter(query, stdin.lock(), stdout.lock()) {
        eprint!("Error: {}", e)
    }
}
