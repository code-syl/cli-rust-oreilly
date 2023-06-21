use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// The files to concat
    #[arg(default_values = ["-"])]
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
                eprintln!("Failed to open {}: {}", file, error),
            Ok(_) => { 
                println!("Opened {}", file);
                print();
            },
        }
    }

    return Ok(());
}

fn open(file_name: &str) -> MyResult<Box<dyn BufRead>> {
    match file_name {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file_name)?))),
    }
}

fn print() {
    println!("Hello, world!");
}