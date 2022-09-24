use super::{CameraAble, ProjectiveCamera};
use crate::{
    core::{
        aabb::Bounds2,
        film::Film,
        medium::Medium,
        ray::{Ray, RayDifferential, RayAble},
        sample::{CameraSample, Sample},
        spectrum::RGBSpectrum,
    },
    extends::*,
    until::transform::Transform,
};
use std::rc::Rc;
///正交投影相机
///
pub struct OrthographicCamera {
    projective: ProjectiveCamera,
    dx_camera: Vec3,
    dy_camera: Vec3,
    index:u32,
}
impl OrthographicCamera {
    pub fn new(
        camera_to_world: Mat4,
        screen_window: Bounds2,
        shutter_open: f32,
        shutter_close: f32,
        lens_radius: f32,
        focal_distance: f32,
        film:&Film,
        medium: Option<Rc<Medium>>,
    ) -> Self {
        //构建一个投影矩阵位正交投影的投影相机。
        let base = ProjectiveCamera::new(
            camera_to_world,
            Mat4::orthographic_lh(-1.0, 1.0, -1.0, 1.0, 0.0, 1.0),
            screen_window,
            shutter_open,
            shutter_close,
            lens_radius,
            focal_distance,
            film,
            medium,
        );
        let dx = base.camera_to_raster.applying_vector(Vec3::X);
        let dy = base.camera_to_raster.applying_vector(Vec3::Y);
        Self {
            projective: base,
            dx_camera: dx,
            dy_camera: dy,
            index:0,
        }
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
impl CameraAble for OrthographicCamera {
    fn generate_ray(&self, sampler: &CameraSample) -> (Option<Ray>, f32) {
        let p_film = Point3::new(sampler.p_film.x, sampler.p_film.y, 0.0);
        let p_camera = self.projective.camera_to_raster.applying_point_inv(p_film);
        let mut ray = Ray::init_o_dir(p_camera, Vec3::Z);
        //相机采样,光圈半径大于0
        if self.projective.len_radius > 0.0 {
            let p_lens = Sample::disk_sample_uniform(&sampler.p_lens) * self.projective.len_radius;
            let t = self.projective.focal_distance / ray.d.z;
            let t = ray.at(t);
            ray.o = Point3::new(p_lens.x, p_lens.y, 0.0);
            ray.d = (t - ray.o).normalize();
        };
        ray.time = lerp(
            sampler.time,
            self.get_shutter_open(),
            self.get_shutter_close(),
        );
        ray.medium = self.get_medium();
        let ray=self.get_camera_to_world().applying_ray(&ray);
        (Some(ray), 1.0)
    }
    fn get_camera_to_world(&self) -> Transform {
        self.projective.get_camera_to_world()
    }
    fn get_medium(&self) -> Option<Rc<Medium>> {
        self.projective.get_medium()
    }
    fn We(&self, ray: &Ray, p_raster2: Option<Point2>) -> RGBSpectrum {
        todo!()
    }

    fn generate_ray_differebtial(&self, sampler: &CameraSample) -> (Option<RayDifferential>, f32) {
        let p_film = Point3::new(sampler.p_film.x, sampler.p_film.y, 0.0);
        let p_camera = self.projective.camera_to_raster.applying_point_inv(p_film);
        let mut ray = Ray::init_o_dir(p_camera, Vec3::Z);
        //相机采样,光圈半径大于0
        if self.projective.len_radius > 0.0 {
            let p_lens = Sample::disk_sample_uniform(&sampler.p_lens) * self.projective.len_radius;
            let t = self.projective.focal_distance / ray.d.z;
            let t = ray.at(t);
            ray.o = Point3::new(p_lens.x, p_lens.y, 0.0);
            ray.d = (t - ray.o).normalize();
        };
        ray.time = lerp(
            sampler.time,
            self.get_shutter_open(),
            self.get_shutter_close(),
        );
        ray.medium = self.get_medium();
        let mut ray_different = RayDifferential::init();
        //求正交的两条辅助光线
        if self.projective.len_radius > 0.0 {
            let p_lens = self.projective.len_radius * Sample::disk_sample_uniform(&sampler.p_lens);
            let ft = self.projective.focal_distance / ray.d.z;
            //对x轴
            let p_focus = p_camera + self.dx_camera + (ft * Vec3::Z);
            ray_different.x_ray_o = Point3::new(p_lens.x, p_lens.y, 0.0);
            ray_different.x_ray_dir = (p_focus - ray_different.x_ray_o).normalize();
            //对y轴
            let p_focus = p_camera + self.dy_camera + (ft * Vec3::Z);
            ray_different.y_ray_o = Point3::new(p_lens.x, p_lens.y, 0.0);
            ray_different.y_ray_dir = (p_focus - ray_different.y_ray_o).normalize();
        } else {
            ray_different.x_ray_o = ray.o + self.dx_camera;
            ray_different.y_ray_o = ray.o + self.dy_camera;
            ray_different.x_ray_dir = ray.d;
            ray_different.y_ray_dir = ray.d;
        };
        ray.time = lerp(
            sampler.time,
            self.get_shutter_open(),
            self.get_shutter_close(),
        );
        ray.medium = self.get_medium();
        ray_different.main_ray = ray;
        ray_different.set_differentials(true);
        (Some(ray_different), 1.0)
    }
    fn get_shutter_close(&self) -> f32 {
        self.projective.base_camera_data.shutter_close
    }
    fn get_shutter_open(&self) -> f32 {
        self.projective.base_camera_data.shutter_open
    }
    fn pdf_we(&self, ray: &Ray, pdf_pos: f32, p_raster: f32) {
        todo!()
    }
}
