use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::{DynamicImage, ImageReader, ImageResult};
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::io::Cursor;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

pub struct CompressImage {
    pub input_path: String,
    pub output_path: String,
    pub quality: u8,
}

impl CompressImage {
    // 压缩图片
    pub fn compress_image(&self) -> Result<(), Box<dyn Error>> {
        // 读图形成DynamicImage
        let image_dynamic = ImageReader::open(&self.input_path)?.decode()?;

        // 创建输出文件
        let file = File::create(&self.output_path)?;
        let mut writer = BufWriter::new(file);

        // 创建JPEG编码器
        let encoder = JpegEncoder::new_with_quality(&mut writer, self.quality);

        image_dynamic.write_with_encoder(encoder)?;

        Ok(())
    }
}
