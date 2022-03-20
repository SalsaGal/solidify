use std::path::PathBuf;

use clap::Parser;
use image::DynamicImage;

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

    for input in args.input {
        if let Ok(image) = image::open(&input) {
            let mut image = image.into_rgba8();
            for pixel in &mut image.chunks_mut(4) {
                if pixel[3] < 255 {
                    pixel[3] = 0;
                }
            }
            image.save_with_format(&output_path, image::ImageFormat::Png).unwrap();
        } else {
            eprintln!("Can't find file {}", input);
        }
    }
}
