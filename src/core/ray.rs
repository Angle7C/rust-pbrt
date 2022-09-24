use std::rc::Rc;

use crate::extends::{Point3, Vec3};

use super::medium::Medium;

pub struct Ray {
    pub o: Point3,
    pub d: Vec3,
    pub t_max: f32,
    pub time: f32,
    pub medium: Option<Rc<Medium>>,
}
pub trait RayAble {
    fn at(&self, t: f32) -> Point3;
    fn has_nans(&self) -> bool;
}
impl Ray {
    pub fn init() -> Self {
        Ray {
            o: (Point3::ZERO),
            d: (Point3::ZERO),
            t_max: (f32::INFINITY),
            time: (f32::INFINITY),
            medium: (None),
        }
    }
    pub fn init_o_dir(o: Point3, d: Vec3) -> Self {
        let mut ray = Self::init();
        ray.o = o;
        ray.d = d.normalize();
        ray
    }
    pub fn new(o: Point3, d: Vec3, t_max: f32, time: f32, medium: Option<Rc<Medium>>) -> Self {
        Self {
            o: o,
            d: d.normalize(),
            t_max: t_max,
            time: time,
            medium: medium,
        }
    }
}
impl RayAble for Ray {
    fn at(&self, t: f32) -> Point3 {
        self.o + t * self.d
    }
    fn has_nans(&self) -> bool {
        self.o.is_nan() || self.d.is_nan()
    }
}
pub struct RayDifferential {
    pub main_ray: Ray,
    pub x_ray_o: Point3,
    pub y_ray_o: Point3,
    pub x_ray_dir: Vec3,
    pub y_ray_dir: Vec3,

    has_differentials: bool,
}
impl RayDifferential {
    pub fn init() -> Self {
        Self {
            main_ray: (Ray::init()),
            x_ray_o: (Point3::ZERO),
            y_ray_o: (Point3::ZERO),
            x_ray_dir: (Vec3::ZERO),
            y_ray_dir: (Vec3::ZERO),
            has_differentials: false,
        }
    }
    pub fn new_o_dir(o: Point3, dir: Vec3) -> Self {
        let mut ray = RayDifferential::init();
        ray.main_ray.o = o;
        ray.main_ray.d = dir;
        ray
    }
    pub fn new(
        o: Point3,
        d: Vec3,
        t_max: f32,
        time: f32,
        medium: Option<Rc<Medium>>,
        x_o: Point3,
        y_o: Point3,
        x_dir: Vec3,
        y_dir: Vec3,
        has: bool,
    ) -> Self {
        Self {
            main_ray: Ray::new(o, d, t_max, time, medium),
            x_ray_o: x_o,
            y_ray_o: y_o,
            x_ray_dir: x_dir,
            y_ray_dir: y_dir,
            has_differentials: has,
        }
    }
    pub fn scale_differentials(&mut self, s: f32) {
        self.x_ray_o = self.x_ray_o + (self.x_ray_o - self.main_ray.o) * s;
        self.y_ray_o = self.y_ray_o + (self.y_ray_o - self.main_ray.o) * s;
        self.x_ray_dir = self.x_ray_dir + (self.x_ray_dir - self.main_ray.d) * s;
        self.x_ray_dir = self.y_ray_dir + (self.x_ray_dir - self.main_ray.d) * s;
    }
    pub fn set_differentials(&mut self,value:bool){ 
        self.has_differentials=value;
    }
}
