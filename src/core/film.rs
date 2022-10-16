use super::aabb::Bound2;
use super::filter::Filter;
use super::spectrum::RGBSpectrum;
use crate::extends::{point2_max, point2_min, Point2, Vector2};
use std::ops::{  Mul};
use std::sync::{ RwLock};
const FILTER_TABLE_WIDTH: usize = 16;

#[derive(Debug, Default, Copy, Clone)]
pub struct FilmTilePixel {
    contrib_sum: RGBSpectrum,
    filter_weight_sum: f64,
}

pub struct FilmTile<'a> {
    pub pixel_bounds: Bound2,
    filter_radius: Vector2,
    inv_filter_radius: Vector2,
    filter_table: &'a [f64; FILTER_TABLE_WIDTH * FILTER_TABLE_WIDTH],
    filter_table_size: usize,
    pixels: Vec<FilmTilePixel>,
    max_sample_luminance: f64,
}
impl<'a> IntoIterator for&'a FilmTile<'a>{
    type IntoIter = FilmTileIterator<'a>;
    type Item = Point2;
    fn into_iter(self) -> Self::IntoIter {
        let p=self.pixel_bounds.min;
        FilmTileIterator{
            p,
            film_tile:self
        }
    }
}
pub struct FilmTileIterator<'a>{
    pub p:Point2,
    pub film_tile:&'a FilmTile<'a>,
}
impl<'a> Iterator for FilmTileIterator<'a>{
    type Item = Point2;
    fn next(&mut self) -> Option<Self::Item> {
       
        if self.p.x==self.film_tile.pixel_bounds.max.x{
            self.p.x=self.film_tile.pixel_bounds.min.x;
            self.p.y+=1.0;
        }else{
            self.p.x+=1.0;
        }
        if self.p.y>self.film_tile.pixel_bounds.max.y{
            None
        }else{
            Some(self.p)
        }
    }
}
impl<'a> FilmTile<'a> {
    pub fn new(
        pixel_bounds: Bound2,
        filter_radius: Vector2,
        filter_table: &'a [f64; FILTER_TABLE_WIDTH * FILTER_TABLE_WIDTH],
        filter_table_size: usize,
        max_sample_luminance: f64,
    ) -> Self {
        let area=pixel_bounds.area();
        FilmTile {
            pixel_bounds,
            filter_radius,
            inv_filter_radius: Vector2 {
                x: 1.0 / filter_radius.x,
                y: 1.0 / filter_radius.y,
            },
            filter_table,
            filter_table_size,
            pixels: vec![FilmTilePixel::default(); area as usize],
            max_sample_luminance,
        }
    }
    pub fn add_sample(&mut self, p_film: Point2, l: &mut RGBSpectrum, sample_weight: f64) {
        //当超过图像最大感受能量，进行归一化。
        if l.y() > self.max_sample_luminance {
            *l = RGBSpectrum::from_value(self.max_sample_luminance / l.y()).mul(*l);
        }
        //
        let p_film_discrete: Point2 = p_film - Vector2 { x: 0.5, y: 0.5 };
        
        let mut p0: Point2 = p_film_discrete - self.filter_radius;
        let mut p1: Point2 = p_film_discrete + self.filter_radius;
        p0 = point2_max(&p0, &self.pixel_bounds.min);
        p1 = point2_min(&p1, &self.pixel_bounds.max);
        let mut ifx: Vec<usize> = Vec::with_capacity(16);
        for x in p0.x as usize..p1.x as usize {
            let fx: f64 = ((x as f64 - p_film_discrete.x)* self.inv_filter_radius.x * self.filter_table_size as f64).abs();
            ifx.push(fx.floor().min(self.filter_table_size as f64 - 1.0) as usize);
        }
        let mut ify: Vec<usize> = Vec::with_capacity(16);
        for y in p0.y as usize..p1.y as usize {
            let fy: f64 = ((y as f64 - p_film_discrete.y)
                * self.inv_filter_radius.y
                * self.filter_table_size as f64)
                .abs();
            ify.push(fy.floor().min(self.filter_table_size as f64 - 1.0) as usize);
        }
        for y in p0.y as usize..p1.y as usize {
            for x in p0.x as usize..p1.x as usize {

                let offset: usize =
                    ify[(y - p0.y as usize)] * self.filter_table_size + ifx[(x - p0.x as usize)];
                let filter_weight: f64 = self.filter_table[offset];

                let idx = self.get_pixel_index(x, y);
                let pixel = &mut self.pixels[idx];
                pixel.contrib_sum = *l
                    * RGBSpectrum::from_value(sample_weight)
                    * RGBSpectrum::from_value(filter_weight)
                    + pixel.contrib_sum;
                pixel.filter_weight_sum += filter_weight;
            }
        }
    }
    fn get_pixel_index(&self, x: usize, y: usize) -> usize {
        let width: usize = (self.pixel_bounds.max.x - self.pixel_bounds.min.x) as usize;
        let pidx =
            (y - self.pixel_bounds.min.y as usize) * width + (x - self.pixel_bounds.min.x as usize);
        pidx as usize
    }
}

pub struct Film {
    ///存储像素
    pub full_resolution: Point2,

    pub diagonal: f64,

    pub filter: Box<Filter>,

    pub filename: String,

    pub cropped_pixel_bounds: Bound2,

    pub pixels: RwLock<image::RgbImage>,
    filter_table: [f64; FILTER_TABLE_WIDTH * FILTER_TABLE_WIDTH],
    // scale: f64,
    max_sample_luminance: f64,
}

impl Film {
    pub fn new(
        //像素值
        resolution : Point2,
        //
        crop_window: Bound2,
        //过滤器
        filter: Box<Filter>,
        //对角线
        diagonal: f64,
        //图像名称
        filename: String,

        // scale: f64,

        max_sample_luminance: f64,
    
    ) -> Self {
        //像素
        let cropped_pixel_bounds = crop_window;
        //过滤器 
        let mut filter_table: [f64; FILTER_TABLE_WIDTH * FILTER_TABLE_WIDTH ] =
            [0.0; FILTER_TABLE_WIDTH * FILTER_TABLE_WIDTH ];
        let mut offset: usize = 0;
        let filter_radius: Vector2 = filter.get_radius();
        //过滤权重表
        for y in 0..FILTER_TABLE_WIDTH {
            for x in 0..FILTER_TABLE_WIDTH {
                //一个像素中的样本点
                let p: Point2 = Point2 {
                    x: (x as f64 + 0.5) * filter_radius.x / FILTER_TABLE_WIDTH as f64,
                    y: (y as f64 + 0.5) * filter_radius.y / FILTER_TABLE_WIDTH as f64,
                };
                filter_table[offset] = filter.evaluate(p);
                offset += 1;
            }
        }
        Film {
            full_resolution: resolution,
            diagonal: diagonal * 0.001,
            filter,
            filename,
            cropped_pixel_bounds,
            pixels: RwLock::new(image::ImageBuffer::new(resolution.x as u32,resolution.y as u32)),
            filter_table,
            // scale,
            max_sample_luminance,
        }
    }
    pub fn get_cropped_pixel_bounds(&self) -> &Bound2 {
        &self.cropped_pixel_bounds
    }
    //返回超过生成像素的范围。
    pub fn get_sample_bounds(&self) -> Bound2 {
        let f: Point2 = Point2 {
            x: self.cropped_pixel_bounds.min.x as f64,
            y: self.cropped_pixel_bounds.min.y as f64,
        } + Vector2 { x: 0.5, y: 0.5 }
            - self.filter.get_radius();
        let c: Point2 = Point2 {
            x: self.cropped_pixel_bounds.max.x as f64,
            y: self.cropped_pixel_bounds.max.y as f64,
        } - Vector2 { x: 0.5, y: 0.5 }
            + self.filter.get_radius();
        Bound2::new(f, c)
    }
    //生成物理范围
    pub fn get_physical_extent(&self) -> Bound2 {
        let aspect: f64 = self.full_resolution.y as f64 / self.full_resolution.x as f64;
        let x: f64 = (self.diagonal * self.diagonal / (1.0 as f64 + aspect * aspect)).sqrt();
        let y: f64 = aspect * x;
        Bound2::new(Point2::new(-x/2.0,-y/2.0),Point2::new(x/2.0,y/2.0))
    }
    //获取图像上的一部份，用来并行渲染
    pub fn get_film_tile(&self, sample_bounds: &Bound2) -> FilmTile {
        let half_pixel: Vector2 = Vector2 { x: 0.5, y: 0.5 };
        
        let float_bounds: Bound2 = sample_bounds.clone();
        let p_min: Point2 = float_bounds.min - half_pixel - self.filter.get_radius();
        let p0: Point2 = Point2 {
            x: p_min.x.ceil(),
            y: p_min.y.ceil(),
        };
        let p_max: Point2 = float_bounds.max - half_pixel + self.filter.get_radius();
        let p1: Point2 = Point2 {
            x: p_max.x.floor()+1.0,
            y: p_max.y.floor()+1.0,
        };
        //求到一部份的像素边界。
        let tile_pixel_bounds: Bound2 =self.cropped_pixel_bounds.intersect(Bound2::new(p0,p1));

        FilmTile::new(
            tile_pixel_bounds,
            self.filter.get_radius(),
            &self.filter_table,
            FILTER_TABLE_WIDTH,
            self.max_sample_luminance,
        )
    }
    pub fn merge_film_tile(&self, tile: &FilmTile) {
        //遍历每一个像素
        for pixel in &tile.pixel_bounds {
            let idx = tile.get_pixel_index(pixel.x as usize, pixel.y as  usize);
            let tile_pixel = &tile.pixels[idx];
            let mut pixels_write = self.pixels.write().unwrap();
            let merge_pixel = pixels_write.get_pixel_mut(pixel.x as u32, pixel.y as u32);
            //计算RGB的值
            let rgb=tile_pixel.contrib_sum.to_rgb();
            *merge_pixel=rgb;
        }
    }
    pub fn set_image(&self, _img: &[RGBSpectrum]) {
        unimplemented!()
        // let n_pixels: i32 = self.cropped_pixel_bounds.area() as i32;
        // let mut pixels_write = self.pixels.write().unwrap();
        // for i in 0..n_pixels as usize {
        //     let mut merge_pixel = &mut pixels_write[i];
        //     let mut xyz: [f64; 3] = [0.0; 3];
        //     img[i].to_xyz(&mut xyz);
        //     for (i, item) in xyz.iter().enumerate() {
        //         merge_pixel.xyz[i] = *item;
        //     }

        //     merge_pixel.filter_weight_sum = 1.0 as f64;
        //     merge_pixel.splat_xyz[0] = 0.0;
        //     merge_pixel.splat_xyz[1] = 0.0;
        //     merge_pixel.splat_xyz[2] = 0.0;
        // }
    }
    pub fn add_splat(&self, p: Point2, v: &RGBSpectrum) {
        let mut v: RGBSpectrum = *v;
        // TODO: ProfilePhase pp(Prof::SplatFilm);
        let pi: Point2 = Point2 {
            x: p.x.ceil(),
            y: p.y.ceil(),
        };
        if v.y() > self.max_sample_luminance {
            v = v * self.max_sample_luminance / v.y();
        }
        let xyz: [f64; 3] =v.rgb;
        let mut pixels_write = self.pixels.write().unwrap();
        let pixel = pixels_write.get_pixel_mut(pi.x as u32, pi.y as u32);
        let splat_xyz= &mut pixel.0;
        splat_xyz[0] += xyz[0] as u8;
        splat_xyz[1] += xyz[1] as u8;
        splat_xyz[2] += xyz[2] as u8;
    }
    #[cfg(not(feature = "openexr"))]
    pub fn write_image(&self, _splat_scale: f64) {
        self.pixels.read().unwrap().save(self.filename.as_str()).unwrap();
    }
}
