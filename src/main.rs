use std::env;
use std::io::{self, BufRead, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    println!("Will search for {}", query);

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        writeln!(stdout, "{}", line).expect("Failed to write line");
    }
}
