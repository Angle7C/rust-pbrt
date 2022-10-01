use std::rc::Rc;

use cgmath::InnerSpace;

use super::{medium::Medium, ray::Ray};
use crate::extends::{Point3, Vector3};
pub trait InteractionAble {
    fn is_surface_interaction(&self) -> bool;
    fn spawn_ray(&self, d: &Vector3) -> Ray;
    fn spawn_ray_point_to(&self, p2: &Vector3) -> Ray;
    fn spawn_ray_interaction_to(&self, it: &Interaction) -> Ray;
    fn is_medium_interaction(&self) -> bool;
    fn get_medium_w(&self, w: &Vector3) -> Box<Medium>;
    fn get_medium(&self) -> Box<Medium>;
}
#[derive(Clone)]
pub struct Interaction {
    //击中的点
    pub p: Point3,
    //光线击中时间
    pub time: f64,
    //与入射光线相反
    pub wo: Vector3,
    //该点的法向
    pub normal: Vector3,
    // 表面材质
    pub medium: Option<Medium>,

    pub surface_interaction: Option<SurfaceInteraction>,
}
impl Interaction {
    // p 和 t 是必须的
    pub fn new(p: Point3, t: f64, w: Vector3, normal: Vector3, medium: Option<Medium>,surface_interaction: Option<SurfaceInteraction>) -> Self {
        Self {
            p: p,
            time: t,
            wo: w,
            normal: normal.normalize(),
            medium: medium,
            surface_interaction: surface_interaction,
        }
    }
    pub fn init() -> Self {
        unimplemented!()
    }
}
#[derive(Clone,Debug)]
pub struct SurfaceInteraction {
    pub dpdu: Vector3,
    pub dpdv: Vector3,
    pub dndu: Vector3,
    pub dndv: Vector3,
}
impl SurfaceInteraction {
    pub fn init(dpdu: Vector3, dpdv: Vector3, dndu: Vector3, dndv: Vector3) -> Self {
        Self {
            dpdu: (dpdu),
            dpdv: (dpdv),
            dndu: (dndu),
            dndv: (dndv),
        }
    }
}
