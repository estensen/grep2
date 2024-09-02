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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_run_grep() {
        let test_cases = [
            (
                "Rust",
                "In the world of code so vast and wide,\nRust stands strong, with memory on its side.",
                "Rust stands strong, with memory on its side.\n",
            ),
            (
                "Rust",
                "No nulls to fear, no data race,\nSafety and speed in every trace.",
                "",
            ),
            (
                "Rust",
                "In Rust we trust, a language to love.\nCompile time checks, borrow and own,\nIn Rust's embrace, we find our home.",
                "In Rust we trust, a language to love.\nIn Rust's embrace, we find our home.\n"
            )
        ];

        for (query, input, expected) in &test_cases {
            let mut output = Vec::new();

            run_grep(query, Cursor::new(input), &mut output).expect("run_grep failed");

            let output_str = String::from_utf8(output).expect("Failed to convert output to string");
            assert_eq!(output_str, *expected)
        }
    }
}
