use crate::extends::{Point2, Vec2};

use super::{aabb::{Bounds3, Bounds2}, filter::FilterAble, spectrum::RGBSpectrum};
struct Pixel {
    pub xyz: [f32; 3],
    pub filter_weight_sum: f32,
    pub pad: f32,
}
struct FileTilePixel{
    filter_weight_sum:f32,
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
pub struct FilmTile{
    pixel_bounds :Bounds2,
    filter_radius:Vec2,
    inv_filter_radius:Vec2,
    filter_table:f32,
    pixels:Vec<FileTilePixel>,
    max_sample_luminance :f32,
}
pub struct Film {
    pub full_Resolution: Point2,
    pub diagonal: f32,
    pub filter: Box<dyn FilterAble>,
    pub filename: &'static str,
    pub cropped_pixel_bounds:Bounds2,
    pixels: Box<Vec<Pixel>>,
    filter_table: i32,
    scale: i32,
    max_sample_luminance: i32,
}
impl Film {
    fn new(
        resolution: &Point2,
        crop_window: &Bounds2,
        filt: Box<dyn FilterAble>,
        diagonal :f32,
        contrast: f32,
        filename: &'static str,
        scale: f32,
        max_sampel_luminance:f32,
    ) -> Self {
        todo!()
    }
}

pub trait FilmAble {
    fn get_sample_bounds(&self)->Bounds2;
    fn get_physical_extent(&self)->Bounds2;
    fn get_film_tile(&self)->Box<FilmTile>;
    fn merge_film_tile(&self,tile :Box<FilmTile>);
    fn set_image(&self,imag:RGBSpectrum);
    fn add_splat(&self,v :RGBSpectrum, p:&Point2);
    fn write_image(&self,splat_scale:f32);
    fn clear(&self);
    fn get_pixel(&self,p:&Point2)->Pixel;
}