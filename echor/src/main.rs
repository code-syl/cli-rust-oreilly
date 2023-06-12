use clap::Parser;

const NEWLINE: char = '\n';

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The text to display
    #[arg(required = true)]
    text: Vec<String>,

    /// Omit the trailing newline
    #[arg(short = 'n')]
    omit_newline: bool,
}

fn main() {
    let args: Args = Args::parse();
    let mut text: String = args.text.join(" ");

    if !args.omit_newline {
        text.push(NEWLINE);
    }

    print!("{}", text);
}