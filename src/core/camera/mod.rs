use self::perspecttivecamera::PerspectiveCamera;

use super::{sample::CameraSample, ray::Ray, spectrum::RGBSpectrum};

pub mod perspecttivecamera;
pub enum Camera{
    Nil,
    Orthographic(),
    Perspective(Box<PerspectiveCamera>),
}
impl Camera{
    pub fn next_camsample(&mut self)->Option<super::sample::CameraSample>{
        match self {
            Self::Perspective(v)=>{
                v.next_camsample()
            }
            _=>unimplemented!()
        }
    }
    pub fn generate_ray(&self,sample:&CameraSample)->Ray{
        match self {
            Self::Perspective(v)=>{
                v.generate_ray(sample)
            },
            _=>unimplemented!()
        }
    }
    pub fn set_pixel(&mut self,sample:&CameraSample,rgb:RGBSpectrum){
        match self{
            Self::Perspective(v)=>{
                v.set_pixel(sample, rgb);
            },
            _=>unimplemented!()
        }
    }
    pub fn output_image(&self){
        match self {
            Self::Perspective(v)=>{
                v.film.output_image();
            }
            _=>unimplemented!()
        }
    }
}