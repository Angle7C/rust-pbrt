use std::rc::Rc;
use crate::extends::*;
use crate::core::*;
use super::{medium::{*, self}, ray::{Ray, RayDifferential}, spectrum::RGBSpectrum};
use super::sample::*;
 struct BaseCamera{
    pub camera_to_world: Mat4,
    pub animated_transform :Mat4,
    pub shutter_open :f32,
    pub shutter_close:f32,
    pub medium : Option<Rc<Medium>>
}
pub struct  CameraSample{
   pub p_film :Point2,
   pub p_lens:Point2,
   pub time:f32,
}
pub trait CameraAble  {
   //基础行为
   fn get_camera_to_world(&self)->Mat4;
   fn get_animated_transform(&self)->Mat4;
   fn get_shutter_close(&self)->f32;
   fn get_shutter_open(&self)->f32;
   fn get_medium(&self)->Option<Rc<Medium>>;
    //给定样本，生成一条光线，
   fn generate_ray(&self,sampler :&CameraSample)->Ray;
   fn generate_ray_differebtial(&self,CameraSample:&Sample)->RayDifferential;
   fn We(&self,ray :&Ray,p_Raster2 :Option<Point2>)->RGBSpectrum;
   fn pdf_we(&self,ray :&Ray,pdf_pos:f32,p_raster:f32);
   //采样wi
   //fn sample_wi(refs :&Interaction,u:&Point2,wi:&Vec3,pdf:f32,p_raster:Option<Point2>)
}
impl CameraAble for BaseCamera{
    fn get_camera_to_world(&self)->Mat4 {
        self.camera_to_world
    }
    fn get_animated_transform(&self)->Mat4 {
        self.animated_transform
    }
    fn get_shutter_close(&self)->f32 {
        self.shutter_close
    }
    fn get_shutter_open(&self)->f32 {
        self.shutter_open
    }
    fn get_medium(&self)->Option<Rc<Medium>> {
        self.medium
    }
    fn generate_ray(&self,sampler :&CameraSample)->Ray {
        todo!();
    }
    fn generate_ray_differebtial(&self,CameraSample:&Sample)->RayDifferential {
        todo!()
    }
    fn We(&self,ray :&Ray,p_Raster2 :Option<Point2>)->RGBSpectrum {
        todo!();
    }
    fn pdf_we(&self,ray :&Ray,pdf_pos:f32,p_raster:f32){
        todo!();
    }
    

}
