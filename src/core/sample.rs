use std::f64::consts::PI;

use cgmath::EuclideanSpace;
use rand::Rng;

use crate::extends::{Point2, Point3};
//为一个像素找到一个x的值，使光线通过这个像素的工作。

pub struct CameraSample {
    //film上的一个点
    pub p_film: Point2,
    //在光圈上采样的一个点[0，1）
    pub p_lens: Point2,
    //对时间进行采样
    pub time: f64,
}
impl CameraSample {
    pub fn new(p_film: Point2, time: f64) -> Self {
        let r = rand::thread_rng().gen_range(0.0, 1.0);
        let theta = rand::thread_rng().gen_range(0.0, 1.0);
        Self {
            p_film: (p_film),
            p_lens: (Point2::new(r, theta)),
            time: (time),
        }
    }
    pub fn new_all(p_film: Point2, time: f64, p_lens: Point2) -> Self {
        Self {
            p_film,
            p_lens,
            time,
        }
    }
}
pub struct Sample;
impl Sample {
    //均匀⚪采样
    pub fn disk_sample_uniform(u: &Point2) -> Point2 {
        let r = u[0].sqrt();
        let theta = u[1] * 2.0 * PI;
        Point2::new(r * theta.cos(), r * theta.sin())
    }
    //均匀球采样
    pub fn sphere_sample_uniform(_u: &Point2) -> Point3 {
        todo!();
    }
}

pub enum Sampler {}
pub trait SamplerAble {
    fn get_current_pixel_mut(&mut self) -> &mut Point2;
    fn get_current_pixel_sample_index(&mut self) -> &mut usize;
    fn get_current_pixel_array1_offset(&mut self) -> &mut usize;
    fn get_current_pixel_array2_offset(&mut self) -> &mut usize;
}
impl Sampler {
    ///对提供像素坐标，进行处理，需要调用这个方法、
    pub fn start_pixel(p: &Point2) {}
    #[allow(non_snake_case)]
    ///返回当前样本向量的下一个维度的样本值，
    pub fn get_1D(&self) -> f64 {
        todo!()
    }
    #[allow(non_snake_case)]
    /// Get2D（） 返回下一个两个维度的样本值。
    pub fn get_2D(&self) -> Point2 {
        todo!()
    }
    /// 初始化给定像素的相机采样，
    pub fn gest_camera_sample(&self, p_raster: &Point2) -> CameraSample {
        let _2d = self.get_2D();
        let p_film = Point2::new(p_raster.x + _2d.x, p_raster.y + _2d.y);
        let time = self.get_1D();
        let p_lens = self.get_2D();
        CameraSample::new_all(p_film, time, p_lens)
    }
    #[allow(non_snake_case)]
    pub fn request_1D_array(n: i32) {}
    #[allow(non_snake_case)]
    pub fn request_2D_array(n: i32) {}
    pub fn round_count(n: i32) -> i32 {
        return n;
    }
    #[allow(non_snake_case)]
    pub fn get_1D_array(n: i32) -> i32 {
        todo!()
    }
    #[allow(non_snake_case)]
    pub fn get_2D_array(n: i32) -> Point2 {
        todo!()
    }
    pub fn start_next_sample() -> bool {
        todo!()
    }
    //clone一个采样器用于多线程
    pub fn clone_seed(seed: i32) -> Option<Sampler> {
        todo!()
    }
    pub fn set_sample_number(sample_num: u128) -> bool {
        todo!()
    }
}

pub struct PixelSampler {
    current_pixel: Point2,
    current_pixel_index: usize,
    samples1_array_sizes: Vec<usize>,
    samples2_array_sizes: Vec<usize>,
    sample_array1: Vec<Vec<f64>>,
    sample_array2: Vec<Vec<Point2>>,
    array1_size: usize,
    array2_size: usize,
    samples_per_pixel:f64,
    pub sampler_1: Vec<Vec<f64>>,
    pub sampler_2: Vec<Vec<Point2>>,
    pub current_1_dimension: Vec<Vec<f64>>,
    pub current_2_dimension: Vec<Vec<Point2>>,
    pub rng: rand::ThreadRng,

}
impl SamplerAble for PixelSampler {
    fn get_current_pixel_array1_offset(&mut self) -> &mut usize {
        &mut self.array1_size
    }

    fn get_current_pixel_mut(&mut self) -> &mut Point2 {
        &mut self.current_pixel
    }

    fn get_current_pixel_sample_index(&mut self) -> &mut usize {
        &mut self.current_pixel_index
    }

    fn get_current_pixel_array2_offset(&mut self) -> &mut usize {
        &mut self.array2_size
    }
}
impl PixelSampler {
    fn new(samples_per_pixel: f64, n_sample_dimensions: i32) -> Self {
        let mut sample1 = Vec::new();
        let mut sample2 = Vec::new();

        for i in 0..n_sample_dimensions {
            sample1.push(vec![samples_per_pixel]);
            sample2.push(vec![Point2::new(
                samples_per_pixel ,
                samples_per_pixel ,
            )]);
        }
        Self {
            current_pixel: Point2::origin(),
            current_pixel_index: 0,
            samples1_array_sizes: vec![],
            samples2_array_sizes: vec![],
            sample_array1: vec![],
            sample_array2: vec![],
            array1_size: 0,
            array2_size: 0,
            sampler_1: sample1,
            sampler_2: sample2,
            current_1_dimension: vec![],
            current_2_dimension: vec![],
            samples_per_pixel:samples_per_pixel,
            rng: (rand::thread_rng()),
        }
    }
    fn start_next_sample(&self) {
        unimplemented!()
    }
    fn set_sample_number(t: u128) -> bool {
        unimplemented!()
    }
    #[allow(non_snake_case)]
    fn get_1D() -> f64 {
        unimplemented!()
    }
    #[allow(non_snake_case)]
    fn get_2D() -> f64 {
        unimplemented!()
    }
}
