//Film用来表示最终的图像。

use super::{
    spectrum::RGBSpectrum,
};
use crate::extends::{Point2};
use image::Rgb;
pub struct Pixel {
    pub xyz: [f64; 3],
    pub filter_weight_sum: f64,
    pub pad: f64,
}
impl Pixel {
    pub fn init() -> Self {
        Self {
            xyz: [0.0, 0.0, 0.0],
            filter_weight_sum: 0.0,
            pad: 0.0,
        }
    }
}
pub struct FilmTile {
    x: u32,
    y: u32,
}
impl FilmTile {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}
#[derive(Debug,Clone)]
pub struct Film {
    //图片像素
    pub full_resolution: Point2,
    //图片名称
    pub filename: &'static str,
    //像素存储
    pixels: image::RgbImage,
    index: usize,
}
impl Film {
    #[inline]
    pub fn new(resolution: Point2, filename: &'static str) -> Self {
        let pixel = image::RgbImage::new(resolution[0] as u32, resolution[1] as u32);
        Self {
            full_resolution: (resolution),
            filename: (filename),
            pixels: (pixel),
            index: 0,
        }
    }
    #[inline]
    pub fn set_pixel(&mut self, i: u32, j: u32, rgb: &RGBSpectrum) {
        let pixel = self.pixels.get_pixel_mut(i, j);
        
        *pixel = rgb.to_rgb();
    }
    #[inline]
    pub fn get_width(&self) -> u32 {
        self.full_resolution[0] as u32
    }
    pub fn get_height(&self) -> u32 {
        self.full_resolution[1] as u32
    }
    pub fn get_pixel(&mut self, x: u32, y: u32) -> &mut Rgb<u8> {
        self.pixels.get_pixel_mut(x, y)
    }
    pub fn set_pixel_tile(&mut self, t: FilmTile, rgb: Rgb<u8>) {
        *self.pixels.get_pixel_mut(t.x, t.y) = rgb
    }
    pub fn output_image(&self) {
        self.pixels.save(self.filename).unwrap();
    }
}
impl Iterator for Film {
    type Item = FilmTile;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.full_resolution[0] as usize * self.full_resolution[1] as usize {
            self.index += 1;
            let x = self.index / self.full_resolution[0] as usize;
            let y = self.index % self.full_resolution[0] as usize;
            Some(FilmTile::new(x as u32, y as u32))
        } else {
            None
        }
    }
}
