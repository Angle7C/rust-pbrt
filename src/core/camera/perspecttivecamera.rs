use std::{rc::Rc, collections::btree_map::Iter};

use crate::{
    core::{aabb::Bounds2, medium::Medium, film::Film, ray::{Ray, RayAble, RayDifferential}, sample::{Sample, CameraSample}, spectrum::RGBSpectrum},
    extends::{Mat4, Vec3, Point3, lerp, Point2}, until::transform::Transform,
};

use super::{CameraAble, ProjectiveCamera};

pub struct PerspectiveCamera {
    projective_camera: ProjectiveCamera,
    dx_camera: Vec3,
    dy_camera: Vec3,
    a:f32,
    index: u32,
}
impl PerspectiveCamera {
    pub fn new(
        camera_to_world: Mat4,
        screen_window: Bounds2,
        shutter_open: f32,
        shutter_close: f32,
        lens_radius: f32,
        focal_distance: f32,
        fov: f32,
        film:&Film,
        medium:Option<Rc<Medium>>
       ) -> Self {
        
        let perspective_camera = ProjectiveCamera::new(
            camera_to_world,
            Mat4::perspective_lh(fov, 1.0, 0.0001, 1000.0),
            screen_window,
            shutter_open,
            shutter_close,
            lens_radius,
            focal_distance,
            film,
            medium,
        );
        let dx_camera=perspective_camera.camera_to_raster.applying_vector_inv(Vec3::X);
        let dy_camera=perspective_camera.camera_to_raster.applying_vector_inv(Vec3::Y);
        //
        let res=film.full_Resolution;
        //在z=1是的图像平面
        let p_min=perspective_camera.camera_to_raster.applying_point_inv(Vec3::ZERO);
        let p_max=perspective_camera.camera_to_raster.applying_point_inv(Point3::new(res.x,res.y,0.0));
        let area=((p_max.x-p_min.x)*(p_max.y-p_min.y)).abs();
        Self { projective_camera: (perspective_camera.into()), dx_camera: (dx_camera.into()), dy_camera: (dy_camera.into()), a: (area.into()),index:0 }
    }
    pub fn next(&mut self,film :&Film)->Option< CameraSample>{
        if self.index as f32 >=film.full_Resolution.x*film.full_Resolution.y{
           return None
        }
        let x=self.index/film.full_Resolution.x as u32;
        let y=self.index%film.full_Resolution.x as u32;
        self.index+=1;
        Some(CameraSample::new(Point2::new(x as f32,y as f32), 0.0))
    }
}
impl CameraAble for PerspectiveCamera {
fn We(
        &self,
        ray: &crate::core::ray::Ray,
        p_raster2: Option<crate::extends::Point2>,
    ) -> crate::core::spectrum::RGBSpectrum {
        RGBSpectrum::new(0.0, 0.0, 0.0)
    }
    fn generate_ray(&self, sampler: &super::CameraSample) -> (Option<Ray>,f32) {

       let p_film=Point3::new(sampler.p_film.x,sampler.p_film.y,0.0);
       let p_camera=self.projective_camera.camera_to_raster.applying_point_inv(p_film);
       let mut ray=Ray::new(Point3::ZERO,p_camera.normalize(),f32::INFINITY,0.0,None);
        //光圈采样
       if self.projective_camera.len_radius>0.0{
        let p_lens=self.projective_camera.len_radius*Sample::disk_sample_uniform(&sampler.p_lens);
        let t=self.projective_camera.focal_distance/ray.d.z;
        let p_focus=ray.at(t);
        ray.o=Point3::new(p_lens.x, p_lens.y,    0.0);
        ray.d=(p_focus-ray.o).normalize();
       }
       ray.time=lerp(sampler.time, self.get_shutter_open(), self.get_shutter_close());
       ray.medium=self.get_medium();
       ray.o =self.get_camera_to_world().applying_point(ray.o);
       ray.d=self.get_camera_to_world().applying_vector(ray.d).normalize();
       (Some(ray),1.0)
    }
    fn generate_ray_differebtial(
        &self,
        sampler: &CameraSample
    ) -> (Option<RayDifferential>,f32) {
        let p_film=Point3::new(sampler.p_film.x,sampler.p_film.y,0.0);
        let p_camera=self.projective_camera.camera_to_raster.applying_point_inv(p_film);
        let mut ray=Ray::new(Point3::ZERO,p_camera.normalize(),f32::INFINITY,0.0,None);
         //光圈采样
        if self.projective_camera.len_radius>0.0{
         let p_lens=self.projective_camera.len_radius*Sample::disk_sample_uniform(&sampler.p_lens);
         let t=self.projective_camera.focal_distance/ray.d.z;
         let p_focus=ray.at(t);
         ray.o=Point3::new(p_lens.x, p_lens.y,    0.0);
         ray.d=(p_focus-ray.o).normalize();
        }

        let mut ray_different = RayDifferential::init();
        //求正交的两条辅助光线
        if self.projective_camera.len_radius > 0.0 {
            let p_lens = self.projective_camera.len_radius * Sample::disk_sample_uniform(&sampler.p_lens);
            let dx=(p_camera+self.dx_camera).normalize();
            let ft = self.projective_camera.focal_distance / dx.z;
            //对x轴
            let p_focus = Point3::ZERO +  (ft * dx);
            ray_different.x_ray_o = Point3::new(p_lens.x, p_lens.y, 0.0);
            ray_different.x_ray_dir = (p_focus - ray_different.x_ray_o).normalize();
            //对y轴
            let dy=(p_camera+self.dy_camera).normalize();
            let ft=self.projective_camera.focal_distance/dy.z;
            let p_focus = Point3::ZERO + self.dy_camera + (ft * dy);
            ray_different.y_ray_o = Point3::new(p_lens.x, p_lens.y, 0.0);
            ray_different.y_ray_dir = (p_focus - ray_different.y_ray_o).normalize();
        } else {
            ray_different.x_ray_o = ray.o ;
            ray_different.y_ray_o = ray.o ;
            ray_different.x_ray_dir = (ray.o+self.dx_camera);
            ray_different.y_ray_dir = (ray.o+self.dy_camera);
        };
        ray_different.set_differentials(true);
        (Some(ray_different),1.0)
    }
    fn get_camera_to_world(&self) -> Transform {
        self.projective_camera.get_camera_to_world()    
    }
    fn get_medium(&self) -> Option<Rc<Medium>> {
        self.projective_camera.get_medium()
    
    }
    fn get_shutter_close(&self) -> f32 {
        self.projective_camera.get_shutter_close()
    }
    fn get_shutter_open(&self) -> f32 {
        self.projective_camera.get_shutter_open()
    }
    fn pdf_we(&self, ray: &crate::core::ray::Ray, pdf_pos: f32, p_raster: f32) {
        todo!();
    }
}
