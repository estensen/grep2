use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        writeln!(stdout, "{}", line).expect("Failed to write line");
    }
}
