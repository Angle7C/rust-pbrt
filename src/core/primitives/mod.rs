use std::rc::Rc;

use PMXUtil::types::Material;

use crate::extends::*;

use super::{aabb::Bounds3, ray::Ray, interaction::SurfaceInteraction, arealight::AreaLight, shape::BaseShapeAble};
pub trait Primitive {
    fn world_bounds(&self)->Bounds3;
    fn intersect(&self,ray:&Ray)->Option<SurfaceInteraction>;
    fn intersect_p(&self,ray:&Ray)->bool; 
    fn get_area_light(&self)->Option<&Rc<AreaLight>>;
    fn get_material(&self)->Option<&Rc<Material>>;
    // fn compute_scattering_functions();
}
pub struct BasePrimitive{
    pub shape_index:usize,
    pub material :Option<Rc<Material>>,
    pub area_light:Option<Rc<AreaLight>>
}
pub struct Aggregate{
    pub list :Box<Vec<Box<dyn BaseShapeAble>>>,
    
}