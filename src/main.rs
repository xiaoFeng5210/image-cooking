use clap::Parser;
use std::io::Cursor;
use std::thread::sleep;
use std::time::Duration;
use image::{ImageReader, DynamicImage, ImageResult};
use image::imageops::FilterType;
use std::io::BufWriter;
use std::fs::File;
use std::time::Instant;


#[derive(Parser)]
#[command(name = "youerning")]
#[command(author = "youerning.top")]
#[command(version = "1.0")]
#[command(about = "a tutorial of crate clap", long_about = None)]
struct Cli {
    input_path: String,
    output_path: String,
}


fn main() -> ImageResult<DynamicImage> {
    let quantity = 80;
    let resize_size = (500, 500);
    let output_path = "./output.jpeg";
    // let image_dynamic = ImageReader::open("./lebai_logo.png")?.decode()?;

    let image_data = ImageReader::open("./lebai_logo.png")?;



    
}
