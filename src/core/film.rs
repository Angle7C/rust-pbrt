//Film用来表示最终的图像。

use image::Rgb;
use crate::extends::{Point2, Vec2};
use super::{
    aabb::{Bounds2, Bounds3},
    filter::FilterAble,
    spectrum::RGBSpectrum,
};
pub struct Pixel {
    pub xyz: [f32; 3],
    pub filter_weight_sum: f32,
    pub pad: f32,
}
struct FileTilePixel {
    filter_weight_sum: f32,
    // contrib_sum Soectrum,
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
    pixel_bounds: Bounds2,
    filter_radius: Vec2,
    inv_filter_radius: Vec2,
    filter_table: f32,
    pixels: Vec<FileTilePixel>,
    max_sample_luminance: f32,
}
pub struct Film {
    //图片像素
    pub full_Resolution: Point2,
    //图片名称
    pub filename: &'static str,
    //像素存储
    pixels: image::RgbImage,
}
impl Film {
    #[inline]
    pub fn new(
        resolution: Point2,
        filename: &'static str,
    ) -> Self {
        let pixel=image::RgbImage::new(resolution.x as u32,resolution.y as u32);
        Self { full_Resolution: (resolution), filename: (filename), pixels: (pixel) }
        
    }
    #[inline]
    fn set_pixel(&mut self,i:u32,j:u32,RGB:&RGBSpectrum){
        let piexl=self.pixels.get_pixel_mut(i, j);
        todo!();
    }
    #[inline]
    fn get_width(&self)->u32{
        self.full_Resolution.x as u32
    }
    fn get_height(&self)->u32{
        self.full_Resolution.y as u32
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
