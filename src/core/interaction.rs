use core::time;
use hexasphere::BaseShape;
use std::rc::Rc;

use super::{medium::Medium, ray::RayAble, *};
use crate::extends::{Normal, Point3, Vec3, UV};
pub trait InteractionAble {
    fn is_surface_interaction(&self) -> bool;
    fn spawn_ray(&self, d: &Vec3) -> Box<dyn RayAble>;
    fn spawn_ray_point_to(&self, p2: &Vec3) -> Box<dyn RayAble>;
    fn spawn_ray_interaction_to(&self, it: &Interaction) -> Box<dyn RayAble>;
    fn is_medium_interaction(&self) -> bool;
    fn get_medium_w(&self, w: &Vec3) -> Box<Medium>;
    fn get_medium(&self) -> Box<Medium>;
}
pub struct Interaction {
    //击中的点
    pub p: Point3,
    //光线击中时间
    pub time: f32,
    //与入射光线相反
    pub wo: Vec3,
    //该点的法向
    pub normal: Vec3,
    // 表面材质
    pub medium: Option<Rc<Medium>>,
    //误差界限
    //pub p_error
}
impl Interaction {
    // p 和 t 是必须的
    pub fn new(p: Point3, t: f32, w: Vec3, normal: Vec3) -> Self {
        Self {
            p: (p),
            time: (t),
            wo: (w),
            normal: (normal),
            medium: None,
        }
    }
    pub fn init() -> Self {
        todo!()
    }
}
pub struct SurfaceInteraction {
    pub rec: Interaction,
    pub uv: UV,
    pub dpdu: Vec3,
    pub dpdv: Vec3,
    pub dndu: Normal,
    pub dndv: Normal,
    // pub shap: Option<Box<dyn bvh::bounding_hierarchy::BHShape>>,
}
impl SurfaceInteraction {
    pub fn init(
        p: Point3,
        time: f32,
        wo: Vec3,
        normal: Vec3,
        medium: Option<Rc<Medium>>,
        uv: UV,
        dpdu: Vec3,
        dpdv: Vec3,
        dndu: Normal,
        dndv: Normal,
    )->Self {
        let t=Interaction::new(p, time, wo, normal);
        Self { rec: (t), uv: (uv), dpdu: (dpdu), dpdv: (dpdv), dndu: (dndu), dndv: (dndv) }
    }
}
