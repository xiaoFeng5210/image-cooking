use clap::Parser;

mod compress;
use compress::CompressImage;

#[derive(Parser)]
#[command(name = "image-cooking")]
#[command(author = "")]
#[command(version = "1.0")]
#[command(about = "a tool to compress image", long_about = None)]
struct Cli {
    input: String,

    #[arg(short, long)]
    output: String,

    #[arg(short, long, default_value = "80")]
    quality: u8,
}

fn main() {
    let cli = Cli::parse();
    let compressor = CompressImage {
        input_path: cli.input,
        output_path: cli.output,
        quality: cli.quality,
    };

    match compressor.compress_image() {
        Ok(_) => println!("Image compressed successfully"),
        Err(e) => println!("Error compressing image: {}", e),
    }
}
