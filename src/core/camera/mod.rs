use std::sync::Arc;
use self::perspecttivecamera::PerspectiveCamera;

use super::{sample::CameraSample, ray::Ray, spectrum::RGBSpectrum, film::Film};

pub mod perspecttivecamera;
pub enum Camera{
    Nil,
    Orthographic(),
    Perspective(Box<PerspectiveCamera>),
}
impl Camera{

    pub fn generate_ray(&self,sample:&CameraSample)->Ray{
        match self {
            Self::Perspective(v)=>{
                v.generate_ray(sample)
            },
            _=>unimplemented!()
        }
    }
    pub fn generater_ray_differential(&self,sample:&CameraSample)->(Ray,f64){
        let mut ray=Ray::default();
        match self {
            Self::Perspective(v)=>{
                let t=v.generate_ray_differential(sample, &mut ray);
                (ray,t)
            },
            _=>unimplemented!()
        }
    }
    pub fn set_pixel(&mut self,_sample:&CameraSample,_rgb:RGBSpectrum){
        match self{
            Self::Perspective(_v)=>{
                // v.set_pixel(sample, rgb);
                unimplemented!()
            },
            _=>unimplemented!()
        }
    }
    pub fn get_film(&self)->Arc<Film>{
        match &self {
            Camera::Perspective(v)=>v.get_film(),
            _=>todo!()
        }
    }
    pub fn output_image(&self){
        unimplemented!()
    }
  
}