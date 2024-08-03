extern crate image;

use image::{ImageReader, Pixel};
use image::{DynamicImage, GenericImageView};

pub struct Texture {
  image: DynamicImage,
  pub width: u32,
  pub height: u32,
}

impl Texture {
  pub fn new(file_path: &str) -> Texture {
    let img = ImageReader::open(file_path).unwrap().decode().unwrap();
    let width = img.width();
    let height = img.height();
    Texture { image: img, width, height }
  }

  pub fn get_pixel_color(&self, x: u32, y: u32) -> u32 {
    let pixel = self.image.get_pixel(x, y).to_rgb();
    let r = pixel[0];
    let g = pixel[1];
    let b = pixel[2];
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
  }
}