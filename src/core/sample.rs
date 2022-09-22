use std::f32::consts::PI;

use crate::extends::{Point2, Point3};
// unsafe static  mut RAND :rand::ThreadRng=rand::thread_rng();


pub trait SampleAble {
    
}
pub struct CameraSample{
    pub sample :Sample,
    pub p_film :Point2,
    pub p_lens:Point2,
    pub time :f32,
}
pub struct  Sample;
impl Sample{
    //均匀⚪采样
    pub fn disk_sample_uniform(u:&Point2)->Point2{
        let r=u.x.sqrt();
        let theta=u.y*2.0*PI;
        Point2::new(r*theta.cos(), r*theta.sin())
    }
    //均匀球采样
    pub fn sphere_sample_uniform(u:&Point2)->Point3{
        todo!();
    }
}
