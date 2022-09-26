pub mod orthographiccamera;
/// sreen space 是定义在了屏幕坐标系，位于Film平面上。
/// Normalized device coording space坐标系，这是渲染图像实际的坐标，（x，y）-》（0，0）to（1，1）
/// (0,0)是图像的左上角，深度与screen space相同，screen通过线性变换成NDC
/// Raster sapce 几乎和NDC相同，（x，y）-》（0，0） to （x_all，y_all）
/// 通过在图像上采样，并变换成screen space中，生成光线。
pub mod perspecttivecamera;
use super::aabb::Bounds2;
use super::film::Film;
use super::sample::*;
use super::{
    medium::*,
    ray::{Ray, RayDifferential},
    spectrum::RGBSpectrum,
};
use crate::extends::*;
use crate::until::transform::Transform;
use std::rc::Rc;
//相机的基础抽象。
struct BaseCamera {
    //相机空间 to 世界空间
    pub camera_to_world: Transform,
    // pub animated_transform: Mat4,
    // 镜头快门打开时间
    pub shutter_open: f32,
    // 镜头快门关闭时间
    pub shutter_close: f32,
    //所处的材质信息
    pub medium: Option<Rc<Medium>>,
}
impl BaseCamera {
    fn new(
        camera_to_world: Mat4,
        shutter_open: f32,
        shutter_close: f32,
        medium: Option<Rc<Medium>>,
    ) -> Self {
        Self {
            camera_to_world: (Transform::new(camera_to_world)),
            // animated_transform: (animated_transform),
            shutter_open: (shutter_open),
            shutter_close: (shutter_close),
            medium: (medium),
        }
    }
}
pub trait CameraAble {
    //基础行为
    fn get_camera_to_world(&self) -> Transform;
    // fn get_animated_transform(&self) -> Transform;
    fn get_shutter_close(&self) -> f32;
    fn get_shutter_open(&self) -> f32;
    fn get_medium(&self) -> Option<Rc<Medium>>;
    //给定相机采样，生成一条光线，并返回一个浮点值来提供这个光对最终的图像的共享度。
    fn generate_ray(&self, sampler: &CameraSample) -> (Option<Ray>, f32);
    //给定相机样本，生成一偏移条光线,并处理纹理,并返回一个浮点值来提供这个光对最终的图像的共享度。
    fn generate_ray_differebtial(&self, sampler: &CameraSample) -> (Option<RayDifferential>, f32);
    fn We(&self, ray: &Ray, p_raster2: Option<Point2>) -> RGBSpectrum;
    fn pdf_we(&self, ray: &Ray, pdf_pos: f32, p_raster: f32);
    //采样wi
    //fn sample_wi(refs :&Interaction,u:&Point2,wi:&Vec3,pdf:f32,p_raster:Option<Point2>)
}
impl CameraAble for BaseCamera {
    fn get_camera_to_world(&self) -> Transform {
        self.camera_to_world
    }
    fn get_shutter_close(&self) -> f32 {
        self.shutter_close
    }
    fn get_shutter_open(&self) -> f32 {
        self.shutter_open
    }
    fn get_medium(&self) -> Option<Rc<Medium>> {
        match self.medium {
            Some(ref v) => Some(v.clone()),
            None => None,
        }
    }
    fn generate_ray(&self, _sampler: &CameraSample) -> (Option<Ray>, f32) {
        todo!();
    }
    fn generate_ray_differebtial(&self, _campler: &CameraSample) -> (Option<RayDifferential>, f32) {
        todo!()
    }
    fn We(&self, _ray: &Ray, _p_raster2: Option<Point2>) -> RGBSpectrum {
        todo!();
    }
    fn pdf_we(&self, _ray: &Ray, _pdf_pos: f32, _p_raster: f32) {
        todo!();
    }
}
//继承了BaseCamera
/// 使用4*4矩阵在所有空间中相互变换，但不是所有相机都能使用这个方法。
/// 透视投影相机
pub struct ProjectiveCamera {
    //基础数据
    base_camera_data: BaseCamera,
    // 屏幕空间 to 图像空间
    camera_to_raster: Transform,
    //相机空间 to 屏幕空间
    camera_to_screen: Transform,
    //屏幕空间 to 相机空间
    screen_to_raster: Transform,
    //镜头光圈半径
    len_radius: f32,
    //焦距
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
        film: &Film,
        medium: Option<Rc<Medium>>,
    ) -> Self {
        //构建基础数据
        let base = BaseCamera::new(camera_to_world, shutter_open, shutter_close, medium);
        let lens_radius = lensr;
        let focal_distance = focald;
        // 先将film平面平移到左上角,装换到NDC空间，转换到raster空间
        let a = Mat4::from_translation(Vec3::new(-screen_window.min.x, -screen_window.max.y, 0.0));
        let b = Mat4::from_scale(Vec3::new(
            1.0 / (screen_window.max.x - screen_window.min.x),
            1.0 / (screen_window.min.y - screen_window.max.y),
            1.0,
        ));
        let c = Mat4::from_scale(Vec3::new(
            film.full_resolution.x,
            film.full_resolution.y,
            1.0,
        ));
        let screen_to_raster =c*b*a;
        //计算变换矩阵
        let camera_to_raster = screen_to_raster * camera_to_screen;
        let screen_to_raster = Transform::new(screen_to_raster);
        let camera_to_screen = Transform::new(camera_to_screen);
        
        Self {
            base_camera_data: base,
            camera_to_screen: camera_to_screen,
            screen_to_raster: screen_to_raster,
            camera_to_raster: Transform::new(camera_to_raster),
            len_radius: lens_radius,
            focal_distance: focal_distance,
        }
    }
}
impl CameraAble for ProjectiveCamera {
    fn We(&self, _ray: &Ray, _p_raster2: Option<Point2>) -> RGBSpectrum {
        todo!()
    }
    fn generate_ray(&self, _sampler: &CameraSample) -> (Option<Ray>, f32) {
        todo!()
    }
    fn get_camera_to_world(&self) -> Transform {
        self.base_camera_data.camera_to_world
    }
    fn generate_ray_differebtial(&self, _sampler: &CameraSample) -> (Option<RayDifferential>, f32) {
        todo!()
    }
    fn get_medium(&self) -> Option<Rc<Medium>> {
        self.base_camera_data.medium.clone()
    }
    fn get_shutter_close(&self) -> f32 {
        self.base_camera_data.shutter_close
    }
    fn get_shutter_open(&self) -> f32 {
        self.base_camera_data.shutter_open
    }
    fn pdf_we(&self, _ray: &Ray, _pdf_pos: f32, _p_raster: f32) {
        todo!()
    }
}
