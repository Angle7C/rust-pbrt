use std::f32::consts::PI;

use rand::Rng;

use crate::extends::{Point2, Point3};
// unsafe static  mut RAND :rand::ThreadRng=rand::thread_rng();


pub trait SampleAble {
        
}
//为一个像素找到一个x的值，使光线通过这个像素的工作。

pub struct CameraSample{
    //film上的一个点
    pub p_film :Point2,
    //在光圈上采样的一个点[0，1）
    pub p_lens:Point2,
    //对时间进行采样
    pub time :f32,
}
impl CameraSample {
    pub fn new(p_film:Point2,time:f32)->Self{
        let r=rand::thread_rng().gen_range(0.0, 1.0);
        let theta=rand::thread_rng().gen_range(0.0, 1.0);
        Self { p_film: (p_film), p_lens: (Point2::new(r,theta)), time: (time) }
    }
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
