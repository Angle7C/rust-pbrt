use std::{rc::Rc, f32::consts::PI};
mod sphere;
use crate::{
    extends::{Affine3, Point2, Vec3},
    core::{
        aabb::Bounds,
        hitcord::{Interaction, SurfaceInteraction},
        ray::Ray,
    },
};
//对线编译器。
pub struct  BaseShape{
    pub  obj_to_world : Rc<Affine3>,
    pub  world_to_world : Rc<Affine3>,
    pub  reverse_orientation: bool,
    pub  transform_swap_handedness : bool,
}
pub trait BaseShapeAble {
    fn new_base()->Self;
    fn object_bound(&self) -> Bounds;
    fn object_world_bound(&self) -> Bounds;
    fn intersect(&self, ray: Ray) -> Option<SurfaceInteraction>;
    fn intersect_p(&self, ray: Ray) -> Option<SurfaceInteraction>;
    fn area(&self) -> f32;
    fn sample(&self, u: &Point2) -> Interaction;
    fn pdf(&self, interaction: &Interaction) -> f32 {
        1.0 / self.area()
    }
    fn pdf_iter(&self, interaction: &Interaction, wi: &Vec3) -> f32;
    fn sample_inter(&self, interaction: &Interaction, u: &Point2) -> Interaction {
        self.sample(u)
    }
    fn obj_to_world(&self) -> Rc<Affine3>;
    fn world_to_world(&self) -> Rc<Affine3>;
    fn reverse_orientation(&self) -> bool;
    //手性交换
    fn transform_swap_handedness(&self) -> bool;
}

