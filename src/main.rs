use std::io::Read;

use clap::Parser;
use itertools::Itertools;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Separator between the words
    #[arg(short, long)]
    separator: String,
}

fn main() {
    let args = Args::parse();

    let mut input = Vec::new();
    let input_size = std::io::stdin().read_to_end(&mut input).unwrap();

    let text = String::from_utf8(input).unwrap();

    let lines = text.lines();

    let mut output = String::with_capacity(input_size);
    for line in lines {
        let mut parts: Vec<&str> = line.split(&args.separator).collect();
        parts.sort();
        let line = parts.iter().join(&args.separator);

        output.push_str(&line);
        output.push('\n')
    }

    print!("{output}");
}
