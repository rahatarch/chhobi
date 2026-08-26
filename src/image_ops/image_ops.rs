use std::io::Cursor;

use image::DynamicImage;
use image::GenericImageView;
use image::ImageFormat;
use image::imageops::FilterType;

pub fn crop_to_square(img: &DynamicImage) -> DynamicImage {
    let (w, h) = img.dimensions();
    let size = w.min(h);
    let x = (w - size) / 2;
    let y = (h - size) / 2;
    img.crop_imm(x, y, size, size)
}

pub fn resize_passport(img: &DynamicImage) -> DynamicImage {
    img.resize_exact(600, 600, FilterType::Lanczos3)
}

pub fn resize_stamp(img: &DynamicImage) -> DynamicImage {
    img.resize_exact(300, 300, FilterType::Lanczos3)
}

pub fn encode_image(img: &DynamicImage, format: ImageFormat) -> Result<Vec<u8>, String> {
    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, format).map_err(|e| format!("encode error ({})", e))?;
    Ok(buf.into_inner())
}