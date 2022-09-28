
use std::rc::Rc;

use cgmath::InnerSpace;

use super::{medium::Medium, ray::Ray,};
use crate::extends::{ Point3, Vector3};
pub trait InteractionAble {
    fn is_surface_interaction(&self) -> bool;
    fn spawn_ray(&self, d: &Vector3) -> Ray;
    fn spawn_ray_point_to(&self, p2: &Vector3) -> Ray;
    fn spawn_ray_interaction_to(&self, it: &Interaction) ->Ray;
    fn is_medium_interaction(&self) -> bool;
    fn get_medium_w(&self, w: &Vector3) -> Box<Medium>;
    fn get_medium(&self) -> Box<Medium>;
}
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
    pub medium: Option<Rc<Medium>>,
    //误差界限
    //pub p_error
}
impl Interaction {
    // p 和 t 是必须的
    pub fn new(p: Point3, t: f64, w: Vector3, normal: Vector3) -> Self {
        Self {
            p: (p),
            time: (t),
            wo: (w),
            normal: (normal).normalize(),
            medium: None,
        }
    }
    pub fn init() -> Self {
        todo!()
    }
}
pub struct SurfaceInteraction {
    pub rec: Interaction,
    pub point3: Point3,
    pub dpdu: Vector3,
    pub dpdv: Vector3,
    pub dndu: Vector3,
    pub dndv: Vector3,
    // pub shap: Option<Box<dyn bvh::bounding_hierarchy::BHShape>>,
}
impl SurfaceInteraction {
    pub fn init(
        p: Point3,
        time: f64,
        wo: Vector3,
        normal: Vector3,
        _medium: Option<Rc<Medium>>,
        point3: Point3,
        dpdu: Vector3,
        dpdv: Vector3,
        dndu: Vector3,
        dndv: Vector3,
    )->Self {
        let t=Interaction::new(p, time, wo, normal);
        Self { rec: (t), point3: (point3), dpdu: (dpdu), dpdv: (dpdv), dndu: (dndu), dndv: (dndv) }
    }
}
