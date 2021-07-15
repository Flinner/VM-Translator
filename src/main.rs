mod hack_binary;
mod parse;
mod types;
mod write_hack;

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Exepected exactly 1 argument")
    }

    let lines = fs::read_to_string(&args[1]).expect("Cann't read file!");

    for (i, line) in lines.lines().enumerate() {
        let parsed = parse::parse_line(line);
        let parsed = match parsed {
            Ok(Some(parsed)) => parsed,
            Ok(None) => continue,
            Err(e) => panic!("{}", e),
        };
        println!("// {}", line);

        let hack = write_hack::convert(parsed, i);
        println!("{}", hack);
    }
}
