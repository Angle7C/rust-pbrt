pub mod perspecttivecamera;
use std::rc::Rc;

use super::aabb::Bounds2;
use super::film::Film;
use super::sample::*;
use super::{
    medium::*,
    ray::{Ray, RayDifferential},
    spectrum::RGBSpectrum,
};
use crate::extends::*;
struct BaseCamera {
    pub camera_to_world: Mat4,
    // pub animated_transform: Mat4,
    pub shutter_open: f32,
    pub shutter_close: f32,
    pub medium: Option<Rc<Medium>>,
}
impl BaseCamera {
    fn new(
        camera_to_world: Mat4,
        // animated_transform: Mat4,
        shutter_open: f32,
        shutter_close: f32,
        medium: Option<Rc<Medium>>,
    ) -> Self {
        Self {
            camera_to_world: (camera_to_world),
            // animated_transform: (animated_transform),
            shutter_open: (shutter_open),
            shutter_close: (shutter_close),
            medium: (medium),
        }
    }
}
pub struct CameraSample {
    pub p_film: Point2,
    pub p_lens: Point2,
    pub time: f32,
}
pub trait CameraAble {
    //基础行为
    fn get_camera_to_world(&self) -> Mat4;
    fn get_animated_transform(&self) -> Mat4;
    fn get_shutter_close(&self) -> f32;
    fn get_shutter_open(&self) -> f32;
    fn get_medium(&self) -> Option<Rc<Medium>>;
    //给定样本，生成一条光线，
    fn generate_ray(&self, sampler: &CameraSample) -> Ray;
    fn generate_ray_differebtial(&self, CameraSample: &Sample) -> RayDifferential;
    fn We(&self, ray: &Ray, p_Raster2: Option<Point2>) -> RGBSpectrum;
    fn pdf_we(&self, ray: &Ray, pdf_pos: f32, p_raster: f32);
    //采样wi
    //fn sample_wi(refs :&Interaction,u:&Point2,wi:&Vec3,pdf:f32,p_raster:Option<Point2>)
}
impl CameraAble for BaseCamera {
    fn get_camera_to_world(&self) -> Mat4 {
        self.camera_to_world
    }
    fn get_animated_transform(&self) -> Mat4 {
        todo!()
    }
    fn get_shutter_close(&self) -> f32 {
        self.shutter_close
    }
    fn get_shutter_open(&self) -> f32 {
        self.shutter_open
    }
    fn get_medium(&self) ->Option<Rc<Medium>> {
        match self.medium {
            Some(ref v)=>Some(v.clone()),
            None=>None
        }
    }
    fn generate_ray(&self, sampler: &CameraSample) -> Ray {
        todo!();
    }
    fn generate_ray_differebtial(&self, CameraSample: &Sample) -> RayDifferential {
        todo!()
    }
    fn We(&self, ray: &Ray, p_Raster2: Option<Point2>) -> RGBSpectrum {
        todo!();
    }
    fn pdf_we(&self, ray: &Ray, pdf_pos: f32, p_raster: f32) {
        todo!();
    }
}
pub struct ProjectiveCamera {
    base_camera_data: BaseCamera,
    camera_to_screen: Mat4,
    raster_to_camera: Mat4,
    screen_to_raster: Mat4,
    raster_to_screen: Mat4,
    len_radius: f32,
    focal_distance: f32,
}
impl ProjectiveCamera {
    fn new(
        camera_to_world: Mat4,
        camera_to_screen: Mat4,
        screen_window: Bounds2,
        shutter_open: f32,
        shutter_close: f32,
        lensr: f32,
        focald: f32,
        film: Option<&Box<Film>>,
        medium: Option<Rc<Medium>>,
    ) -> Self {
        let base = BaseCamera::new(camera_to_world, shutter_open, shutter_close, medium);
        let lens_radius = lensr;
        let focal_distance = focald;
        let film = film.unwrap();
        let screen_to_raster =
            Mat4::from_scale(Vec3::new(
                film.full_Resolution.x,
                film.full_Resolution.y,
                1.0,
            )) * Mat4::from_scale(Vec3::new(
                1.0 / (screen_window.max.x - screen_window.min.x),
                1.0 / (screen_window.min.y - screen_window.max.y),
                1.0,
            )) * Mat4::from_translation(Vec3::new(-screen_window.min.x, -screen_window.max.y, 0.0));
        let raster_to_screen = screen_to_raster.inverse();
        let raster_to_camera = camera_to_screen.inverse() * raster_to_screen;
        Self {
            base_camera_data: (base),
            camera_to_screen: (camera_to_screen),
            raster_to_camera: (raster_to_camera),
            screen_to_raster: (screen_to_raster),
            raster_to_screen: (raster_to_screen),
            len_radius: (lens_radius),
            focal_distance: (focal_distance),
        }
    }
}
