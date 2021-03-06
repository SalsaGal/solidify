use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Files to convert
    input: Vec<String>,

    /// Destination of files (defaults to working directory)
    #[clap(short)]
    output: Option<String>,

    /// Retain colour if the pixel is greater than this value
    #[clap(short)]
    threshold: Option<u8>,
}

fn main() {
    let args = Args::parse();
    let output_path = if let Some(path) = args.output {
        PathBuf::from(path)
    } else {
        std::env::current_dir().unwrap()
    };

    for input in args.input {
        let input = PathBuf::from(input);
        if let Ok(image) = image::open(&input) {
            let mut image = image.into_rgba8();
            for pixel in &mut image.chunks_mut(4) {
                let transparancy = &mut pixel[3];
                if *transparancy > args.threshold.unwrap_or(0) {
                    *transparancy = 255;
                } else {
                    *transparancy = 0;
                }
            }
            let mut path = output_path.clone();
            if path.is_dir() {
                path.push(input.file_name().unwrap());
            }
            image.save_with_format(path, image::ImageFormat::Png).unwrap();
        } else {
            eprintln!("Can't find file {}", input.to_str().unwrap());
        }
    }
}
