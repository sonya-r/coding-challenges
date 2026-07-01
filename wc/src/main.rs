use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

struct Commands {
    chars: bool,
    words: bool,
    lines: bool,
}

impl Commands {
    fn new() -> Self {
        Commands {
            chars: true,
            words: true,
            lines: true,
        }
    }

    fn reset(&mut self) {
        self.chars = false;
        self.words = false;
        self.lines = false;
    }

    fn parse_large(&mut self, command: &str) {
        if command == "chars" {
            self.chars = true;
        } else if command == "words" {
            self.words = true;
        } else if command == "lines" {
            self.lines = true;
        } else {
            self.bad_command(command);
        }
    }

    fn parse_short(&mut self, command: char) {
        if command == 'c' {
            self.chars = true;
        } else if command == 'w' {
            self.words = true;
        } else if command == 'l' {
            self.lines = true;
        } else {
            self.bad_command(command);
        }
    }

    fn bad_command<T>(&self, param: T)
    where
        T: Display,
    {
        eprintln!("Bad parameter: {param}");
        exit(1);
    }
}

fn main() {
    let mut args = env::args();
    args.next();

    let mut filesnames: Vec<String> = Vec::new();
    let mut commands = Commands::new();

    for arg in args {
        let leading = arg.get(..=0).unwrap();

        if leading == "-" {
            commands.reset();
            if arg.get(..=1).unwrap() == "--" {
                let command = arg.get(2..).unwrap();
                commands.parse_large(command);
            } else {
                let mut chars = arg.chars();
                chars.next();

                for ch in chars {
                    commands.parse_short(ch);
                }
            }
        } else {
            filesnames.push(arg);
        }
    }

    if filesnames.len() == 0 {
        println!("Filenames not found");
        return;
    }

    for filename in filesnames {
        let file = File::open(&filename);

        match file {
            Result::Err(_) => {
                eprintln!("File not Found");
                break;
            }
            Result::Ok(mut file) => {
                let mut buffer = String::new();
                let _result = file.read_to_string(&mut buffer);
                let mut output: Vec<String> = Vec::new();

                if commands.lines {
                    let lines = buffer.lines().count();
                    output.push(format!("{:>5}", lines))
                }
                if commands.words {
                    let words = buffer.split_whitespace().count();
                    output.push(format!("{:>5}", words))
                }
                if commands.chars {
                    let characters = buffer.chars().count();
                    output.push(format!("{:>5}", characters))
                }

                let output = output.join(" ");

                println!("{output} {filename}");
            }
        };
    }
}
