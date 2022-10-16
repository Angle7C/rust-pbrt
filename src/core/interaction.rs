use cgmath::{EuclideanSpace, InnerSpace, Zero};

use super::{
    medium::Medium, primitives::Primitive, ray::Ray, reflection::bsdf::BSDF, shape::Shape,
};
use crate::extends::{Point2, Point3, Vector3};
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
    //
    pub medium: Option<Medium>,
    // pub surface_interaction: Option<SurfaceInteraction>,
}
impl Interaction {
    // p 和 t 是必须的
    pub fn new(
        p: Point3,
        t: f64,
        w: Vector3,
        normal: Vector3,
        medium: Option<Medium>,
        // surface_interaction: Option<SurfaceInteraction>,
    ) -> Self {
        Self {
            p: p,
            time: t,
            wo: w,
            normal: normal.normalize(),
            medium: medium,
            // surface_interaction: surface_interaction,
        }
    }
    pub fn init() -> Self {
        unimplemented!()
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Shading {
    pub n: Vector3,
    pub dpdu: Vector3,
    pub dpdv: Vector3,
    pub dndu: Vector3,
    pub dndv: Vector3,
}
#[derive(Clone, Debug)]
pub struct SurfaceInteraction<'a> {
    pub p: Point3,
    //光线击中时间
    pub time: f64,
    //与入射光线相反
    pub wo: Vector3,
    //该点的法向
    pub normal: Vector3,
    //
    pub medium: Option<Medium>,
    pub uv: Point2,
    pub dpdu: Vector3,
    pub dpdv: Vector3,
    // pub dndu: Vector3,
    // pub dndv: Vector3,
    // pub dpdx: Vector3,
    // pub dpdy: Vector3,
    // pub dudx: f64,
    // pub dvdx: f64,
    // pub dudy: f64,
    // pub dvdy: f64,
    pub primitive: Option<*const Primitive>,
    pub shading: Shading,
    pub bsdf: Option<BSDF>,
    // pub bssrdf: Option<TabulatedBssrdf>,
    pub shape: Option<&'a Shape>,
}
impl<'a> Default for SurfaceInteraction<'a>{
    fn default() -> Self {
        Self::init()
    }
}
impl<'a> SurfaceInteraction<'a> {
    pub fn init() -> Self {
        Self {
            p: Point3::origin(),
            time: 0.0,
            wo: Vector3::zero(),
            normal: Vector3::zero(),
            medium: None,
            uv: Point2::origin(),
            dpdu: Vector3::zero(),
            dpdv: Vector3::zero(),
            // dndu: Vector3::zero(),
            // dndv: Vector3::zero(),
            // dpdx: Vector3::zero(),
            // dpdy: Vector3::zero(),
            // dudx: 0.0,
            // dvdx: 0.0,
            // dudy: 0.0,
            // dvdy: 0.0,
            primitive: None,
            shading: Shading {
                n: Vector3::zero(),
                dpdu: Vector3::zero(),
                dpdv: Vector3::zero(),
                dndu: Vector3::zero(),
                dndv: Vector3::zero(),
            },
            bsdf: None,
            shape: None,
        }
    }
    pub fn new(p:Point3,time:f64,w0:Vector3,normal:Vector3)->Self{
       let mut obj=Self::init();
       obj.p=p;
       obj.time=time;
       obj.wo=w0;
       obj.normal=normal;
       obj
    }
}
