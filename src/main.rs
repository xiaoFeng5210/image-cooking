use clap::Parser;
use std::fs;

mod compress;
mod utils;
use compress::CompressImage;
use utils::create_output;

#[derive(Parser)]
#[command(name = "image-cooking")]
#[command(author = "")]
#[command(version = "1.0")]
#[command(about = "a tool to compress image", long_about = None)]
struct Cli {
    input: String,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long, default_value = "80")]
    quality: u8,

    #[arg(short, long = "size")]
    max_size: Option<u32>,
}

fn main() {
    let cli = Cli::parse();
    let compressor = CompressImage {
        input_path: cli.input,
        output_path: create_output(cli.output),
        quality: cli.quality,
        max_size: cli.max_size,
    };

    match compressor.compress_image() {
        Ok(_) => {
            // println!("✅ Image compressed successfully");
            let input_size = fs::metadata(&compressor.input_path).unwrap().len();
            let output_size = fs::metadata(&compressor.output_path).unwrap().len();
            if output_size >= input_size {
                // 输出图片比输入图片大，可以采用--size的方式压缩
                println!("⚠️ 压缩后图片比原图大, 原图可能压缩过，可以采用--size的方式压缩");
                println!("压缩后图片大小: {}KB", output_size / 1000);
            } else {
                println!("✅ 压缩成功，压缩后图片大小: {}KB", output_size / 1000);
            }
        }
        Err(e) => println!("❌ Error compressing image: {}", e),
    };
}
