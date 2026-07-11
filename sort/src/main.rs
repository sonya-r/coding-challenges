use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    filename: String,

    #[arg(short, long, default_value_t = false)]
    unique: bool,

    #[arg(short = 'f', long, default_value_t = false)]
    ignore_case: bool,

    #[arg(short, long, default_value_t = false)]
    reverse: bool,
}

fn main() {
    let args = Args::parse();
    let buffer = fs::read_to_string(args.filename).unwrap();
    let mut lines: Vec<_> = buffer.lines().map(|i| i.to_string()).collect();

    if args.ignore_case {
        for line in lines.iter_mut() {
            *line = line.to_lowercase();
        }
    }

    if args.unique {
        lines.sort();
        lines.dedup();
    }

    if args.reverse {
        lines.sort_by(|a, b| b.cmp(a));
    }

    if !args.unique && !args.reverse {
        lines.sort();
    }

    for line in lines {
        println!("{line}")
    }
}
