use std::rc::Rc;

use crate::{
    core::{aabb::Bounds2, medium::Medium, film::Film, ray::{Ray, RayAble}, sample::Sample},
    extends::{Mat4, Vec3, Point3, lerp},
};

use super::{CameraAble, ProjectiveCamera};

pub struct PerspectiveCamera {
    projective_camera: ProjectiveCamera,
    dx_camera: Vec3,
    dy_camera: Vec3,
    a: f32,
}
impl PerspectiveCamera {
    pub fn new(camera_to_world: Mat4,
        screen_window: Bounds2,
        shutter_open: f32,
        shutter_close: f32,
        lens_radius: f32,
        focal_distance: f32,
        fov: f32,
        film:Option<Box<Film>>,
        medium:Option<Rc<Medium>>
       ) -> Self {
        
        let perspective_camera = ProjectiveCamera::new(
            camera_to_world,
            Mat4::perspective_rh_gl(fov, 1.0, 1.0, 1000.0),
            screen_window,
            shutter_open,
            shutter_close,
            lens_radius,
            focal_distance,
            film.as_ref(),
            medium,
        );
        let dx_camera=perspective_camera.raster_to_camera.transform_vector3(Vec3::new(1.0, 0.0, 0.0));
        let dy_camera=perspective_camera.raster_to_camera.transform_vector3(Vec3::new(0.0, 1.0, 0.0));
        let res=film.unwrap().full_Resolution;
        let p_min=perspective_camera.raster_to_camera.transform_point3(Vec3::ZERO);
        let p_max=perspective_camera.raster_to_camera.transform_point3(Point3::new(res.x,res.y,0.0));
        let area=((p_max.x-p_min.x)*(p_max.y-p_min.y)).abs();
        Self { projective_camera: (perspective_camera.into()), dx_camera: (dx_camera.into()), dy_camera: (dy_camera.into()), a: (area.into()) }
    }
}
impl CameraAble for PerspectiveCamera {
    fn We(
        &self,
        ray: &crate::core::ray::Ray,
        p_Raster2: Option<crate::extends::Point2>,
    ) -> crate::core::spectrum::RGBSpectrum {
        todo!()
    }
    fn generate_ray(&self, sampler: &super::CameraSample) -> crate::core::ray::Ray {
       let p_film=Point3::new(sampler.p_film.x,sampler.p_film.y,0.0);
       let p_camera=self.projective_camera.raster_to_camera.transform_point3(p_film);
       let mut ray=Ray::new(Point3::ZERO,p_camera.normalize(),f32::INFINITY,0.0,None);
       if (self.projective_camera.len_radius>0.0){
        let p_lens=self.projective_camera.len_radius*Sample::disk_sample_uniform(&sampler.p_lens);
        let t=self.projective_camera.focal_distance/ray.d.z;
        let p_focus=ray.at(t);
        ray.o=Point3::new(p_lens.x, p_lens.y,    0.0);
        ray.d=(p_focus-ray.o).normalize();
       }
       ray.time=lerp(sampler.time, self.get_shutter_open(), self.get_shutter_close());
       ray.medium=self.get_medium();
       ray.o =self.get_camera_to_world().transform_point3(ray.o);
       ray.d=self.get_camera_to_world().transform_vector3(ray.d).normalize();
       ray
    }
    fn generate_ray_differebtial(
        &self,
        CameraSample: &crate::core::sample::Sample,
    ) -> crate::core::ray::RayDifferential {
        todo!()
    }
    fn get_animated_transform(&self) -> crate::extends::Mat4 {
        todo!()
    }
    fn get_camera_to_world(&self) -> crate::extends::Mat4 {
        todo!()
    }
    fn get_medium(&self) -> Option<Rc<Medium>> {
        todo!()
    
    }
    fn get_shutter_close(&self) -> f32 {
        todo!()
    }
    fn get_shutter_open(&self) -> f32 {
        todo!()
    }
    fn pdf_we(&self, ray: &crate::core::ray::Ray, pdf_pos: f32, p_raster: f32) {
        todo!()
    }
}
