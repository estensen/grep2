use std::env;
use std::io::{self, BufRead, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("microgrep takes one argument");
    }
    let query = &args[1];
    println!("Will search for {}", query);

    let stdin = io::stdin();
    let stdout = io::stdout();

    if let Err(e) = run_grep(query, stdin.lock(), stdout.lock()) {
        eprint!("Error: {}", e)
    }
}

pub fn run_grep<R: BufRead, W: Write>(query: &str, reader: R, mut writer: W) -> io::Result<()> {
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.contains(query) {
            writeln!(writer, "{}", line).expect("Failed to write line");
        }
    }
    Ok(())
}
