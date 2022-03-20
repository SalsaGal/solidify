use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Files to convert
    input: Vec<String>,

    /// Destination of files (defaults to working directory)
    #[clap(short)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();
    let output_path = if let Some(path) = args.output {
        PathBuf::from(path)
    } else {
        std::env::current_dir().unwrap()
    };
}
