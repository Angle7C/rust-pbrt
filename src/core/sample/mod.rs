use self::stratifiedsampler::StratifiedSampler;

pub mod stratifiedsampler;
pub enum Sampler{
    Stratified(StratifiedSampler)
}
use std::f64::consts::PI;

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