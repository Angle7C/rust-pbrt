//Film用来表示最终的图像。

use super::{
    aabb::Bounds2,
    spectrum::RGBSpectrum,
};
use crate::extends::{Point2};
use image::Rgb;
pub struct Pixel {
    pub xyz: [f32; 3],
    pub filter_weight_sum: f32,
    pub pad: f32,
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
        let pixel = image::RgbImage::new(resolution.x as u32, resolution.y as u32);
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
        self.full_resolution.x as u32
    }
    pub fn get_height(&self) -> u32 {
        self.full_resolution.y as u32
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

pub trait FilmAble {
    fn get_sample_bounds(&self) -> Bounds2;
    fn get_physical_extent(&self) -> Bounds2;
    fn get_film_tile(&self) -> Box<FilmTile>;
    fn merge_film_tile(&self, tile: Box<FilmTile>);
    fn set_image(&self, imag: RGBSpectrum);
    fn add_splat(&self, v: RGBSpectrum, p: &Point2);
    fn write_image(&self, splat_scale: f32);
    fn clear(&self);
    fn get_pixel(&self, p: &Point2) -> Pixel;
}
impl Iterator for Film {
    type Item = FilmTile;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.full_resolution.x as usize * self.full_resolution.y as usize {
            self.index += 1;
            let x = self.index / self.full_resolution.x as usize;
            let y = self.index % self.full_resolution.x as usize;
            
            Some(FilmTile::new(x as u32, y as u32))
        } else {
            None
        }
    }
}
