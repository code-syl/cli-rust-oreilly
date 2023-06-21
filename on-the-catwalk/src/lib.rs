use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

const STDIN: &str = "stdin";

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// The files to concat
    #[arg(default_values = [STDIN])]
    files: Vec<String>,

    /// Number the output lines, starting at 1
    #[arg(
        short = 'n', long = "number",
        conflicts_with = "number_nonblank_lines",
    )]
    number_lines: bool,

    /// Number nonblank output lines
    #[arg(short = 'b', long = "number-nonblank")]
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();
    return Ok(config);
}

pub fn run(config: Config) -> MyResult<()> {
    for file in config.files {
        let stream = open(&file);
        match stream {
            Err(error) => 
                eprintln!("# Failed to open {}: {}", file, error),
            Ok(_) => { 
                println!("# Opened {}", file);
                print_lines(
                    stream.unwrap().lines(),
                    config.number_lines,
                    config.number_nonblank_lines
                );
            },
        }
    }

    return Ok(());
}

fn open(file_name: &str) -> MyResult<Box<dyn BufRead>> {
    match file_name {
        STDIN => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file_name)?))),
    }
}

fn print_lines(lines: Lines<Box<dyn BufRead>>,
               number_lines: bool,
               number_nonblank_lines: bool) {
    let mut line_number:i32 = 1;
    for line in lines {
        match line {
            Err(error) => eprintln!("# Failed to read line: {}", error),
            Ok(line) => {
                if (!number_lines || line.is_empty()) && 
                    !number_nonblank_lines {
                    println!("{}", line);
                    continue;
                }

                println!("{}\t{}", line_number, line);
            },
        }

        line_number += 1;
    }
}