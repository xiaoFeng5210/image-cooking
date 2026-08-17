use image::DynamicImage;
use image::GenericImageView;
use image::ImageReader;
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

pub struct CompressImage {
    pub input_path: String,
    pub output_path: String,
    pub quality: u8,
    pub max_size: Option<u32>,
}

impl CompressImage {
    // 压缩图片
    pub fn compress_image(&self) -> Result<(), Box<dyn Error>> {
        // 读图形成DynamicImage
        let image_dynamic = ImageReader::open(&self.input_path)?.decode()?;
        // 创建输出文件
        let file = File::create(&self.output_path)?;
        let mut writer = BufWriter::new(file);

        if let Some(max_size) = self.max_size {
            let resized_image = self.resize_image(max_size)?;
            let encoder = JpegEncoder::new_with_quality(&mut writer, self.quality);
            resized_image.write_with_encoder(encoder)?;
        } else {
            // 创建JPEG编码器
            let encoder = JpegEncoder::new_with_quality(&mut writer, self.quality);
            image_dynamic.write_with_encoder(encoder)?;
        }

        Ok(())
    }

    fn resize_image(&self, max_size: u32) -> Result<DynamicImage, Box<dyn Error>> {
        let img_dynamic = self.create_dynamic_image()?;
        let (w, h) = img_dynamic.dimensions();
        let max_size_resize = calculate_max_size(w, h, max_size);

        let resized_image =
            img_dynamic.resize(max_size_resize, max_size_resize, FilterType::Lanczos3);

        Ok(resized_image)
    }

    fn create_dynamic_image(&self) -> Result<DynamicImage, Box<dyn Error>> {
        let image_dynamic = ImageReader::open(&self.input_path)?.decode()?;
        Ok(image_dynamic)
    }
}

// 计算最大尺寸
fn calculate_max_size(w: u32, h: u32, max_size: u32) -> u32 {
    let max_size_resize: u32;

    if w > h {
        if w > max_size {
            max_size_resize = max_size;
        } else {
            max_size_resize = w;
        }
    } else {
        if h > max_size {
            max_size_resize = max_size;
        } else {
            max_size_resize = h;
        }
    }

    max_size_resize
}

#[cfg(test)]
mod compress_image_tests {
    use super::*;

    #[test]
    fn test_calculate_1() {
        let w = 1000;
        let h = 500;
        let max_size = 500;

        let result = calculate_max_size(w, h, max_size);
        assert_eq!(result, 500)
    }

    #[test]
    fn test_calculate_2() {
        let w = 500;
        let h = 1000;
        let max_size = 500;

        let result = calculate_max_size(w, h, max_size);
        assert_eq!(result, 500)
    }
}
