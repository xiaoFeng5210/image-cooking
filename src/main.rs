use clap::Parser;
use std::io::Cursor;
use image::{ImageReader, DynamicImage, ImageResult};
use image::imageops::FilterType;
use std::io::BufWriter;
use std::fs::File;


#[derive(Parser)]
#[command(name = "youerning")]
#[command(author = "youerning.top")]
#[command(version = "1.0")]
#[command(about = "a tutorial of crate clap", long_about = None)]
struct Cli {
    input_path: String,
    output_path: String,
}




fn main() -> ImageResult<()> {
    let quantity = 80;
    let resize_size = (500, 500);
    let output_path = "./output.jpeg";
    let image_dynamic = ImageReader::open("./lebai_logo.png")?.decode()?;
    let resize_image = image_dynamic.resize(resize_size.0, resize_size.1, FilterType::Lanczos3);


    Ok(())
}
