use std::fs::read_to_string;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    path: String,

    #[clap(short, long)]
    config: Option<String>,
}

fn run_cat(path: &str) {
    match read_to_string(path) {
        Ok(content) => print!("{}", content),
        Err(reason) => println!("{}", reason),
    }
}

fn main() {
    let args = Args::parse();
    run_cat(&args.path);
}

