use std::rc::Rc;
use crate::extends::{ Point3};
use crate::core::bxdf::BxdF;
use super::{
    aabb::Bounds3,
   
    interaction::{Interaction, SurfaceInteraction},
    light::AreaLight,
    light::Light,
    ray::Ray,
    shape::{ Shape}, material::Material,
};
pub mod bvh;
pub struct Primitive {
    //shape索引
    pub shape_index: usize,
    //材质
    pub material: Option<Rc<Material>>,
    //光源
    pub light: Option<Rc<Light>>,
    //反射模型
    pub bxdf:  Option<BxdF>,
}
impl Primitive {
    pub fn new(
        shape_index: usize,
        bxdf: Option< BxdF>,
        materail: Option<Rc<Material>>,
        light: Option<Rc<Light>>,
    ) -> Self {
        Self {
            shape_index: (shape_index),
            material: (materail),
            light: (light),
            bxdf: bxdf,
        }
    }
    pub fn get_bound(&self,shape:&Vec<Shape>,)->Bounds3{
        shape[self.shape_index].object_world_bound()
    }
    pub fn get_center(&self,shape:&Vec<Shape>)->Point3{
        shape[self.shape_index].object_world_bound().center()
        
    }
}
impl Default for Primitive {
    fn default() -> Self {
        Self::new(0,None, None, None)
    }
}
pub trait PrimitiveAble {
    fn world_bounds(&self) -> Bounds3;
    fn intersect(&self, ray: &Ray) -> Option<Interaction>;
    fn intersect_p(&self, ray: &Ray) -> Option<SurfaceInteraction>;
    fn get_area_light(&self) -> Option<&Rc<AreaLight>>;
    fn get_material(&self) -> Option<&Rc<Material>>;
    // fn compute_scattering_functions();
}
pub struct Aggregate {
    pub list: Box<Vec<Shape>>,
}
