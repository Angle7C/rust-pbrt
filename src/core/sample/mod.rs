use self::stratifiedsampler::StratifiedSampler;

pub mod stratifiedsampler;
#[derive(Debug)]
pub enum Sampler{
    Stratified(StratifiedSampler),
    Nil
}
unsafe impl Sync for Sampler{
    
}
unsafe impl Send for Sampler{
    
}
impl Sampler{
    pub fn clone_with_seed(&self)->Self{
        match self {
            Sampler::Stratified(s)=>{
                Sampler::Stratified(s.clone())
            },
            _=>unimplemented!()
        }
    }
    pub fn start_pixel(&mut self,pixel:Point2){
        match self {
            Sampler::Stratified(s)=>{
               s.start_pixel(pixel);
            },
            _=>unimplemented!()
        }
    }
    pub fn start_next_sample(&mut self)->bool{
        unimplemented!()
    }
    pub fn reseed(&mut self,code:u64){
        match self {
            Sampler::Stratified(v)=>v.reseed(code),
            _=>unimplemented!(),
        }
    }
    pub fn get_camera_sample(&mut self,p:Point2)->CameraSample{
        match self {
            Self::Stratified(v)=>{
                let d=v.get_2d();
                let p=Point2::new(p.x+d.x,p.y+d.y);
                CameraSample::new(p,v.get_1d())
            },
            _=>unimplemented!()
        }
    }
    pub fn get_1d(mut self)->f64{
        match self {
            Self::Stratified(v)=>{
                v.get_1d()
            },
            _=>unimplemented!()
        }
    }
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