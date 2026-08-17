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
        let mut actual_w = 0;
        let mut actual_h = 0;

        // 获取图片长宽
        let (w, h) = img_dynamic.dimensions();
        // 宽高比
        let aspect_ratio = w / h;

        if w > h {
            if w > max_size {
                actual_w = max_size;
                actual_h = actual_w / aspect_ratio;
            } else {
                actual_w = w;
                actual_h = actual_w / aspect_ratio;
            }
        } else {
            if h > max_size {
                actual_h = max_size;
                actual_w = actual_h * aspect_ratio;
            } else {
                actual_h = h;
                actual_w = actual_h * aspect_ratio;
            }
        }

        let resized_image = img_dynamic.resize(actual_w, actual_h, FilterType::Lanczos3);

        Ok(resized_image)
    }

    fn create_dynamic_image(&self) -> Result<DynamicImage, Box<dyn Error>> {
        let image_dynamic = ImageReader::open(&self.input_path)?.decode()?;
        Ok(image_dynamic)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_add() {
//         assert_eq!(add(1, 2), 3);
//     }
// }
