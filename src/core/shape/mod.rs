use std::{f32::consts::PI, rc::Rc};
pub mod sphere;
use crate::{
    core::{
        aabb::Bounds3,
        interaction::{Interaction, SurfaceInteraction},
        ray::Ray,
    },
    extends::{Affine3, Mat4, Point2, Vec3},
};
//对线编译器。
pub struct BaseShape {
    pub obj_to_world: Mat4,
    pub world_to_object: Mat4,
    pub reverse_orientation: bool,
    pub transform_swap_handedness: bool,
}
impl BaseShape {
    pub fn new(object_to_world: Mat4, world_to_object: Mat4, reverse_orientation: bool) -> Self {
        Self {
            obj_to_world: (object_to_world),
            world_to_object: (world_to_object),
            reverse_orientation: (reverse_orientation),
            transform_swap_handedness: (false),
        }
    }
    fn obj_to_world(&self) -> Mat4{
        self.obj_to_world
    }
    fn world_to_world(&self) -> Mat4{
        self.world_to_object
    }
    fn reverse_orientation(&self) -> bool{
        self.reverse_orientation
    }
    //手性交换
    fn transform_swap_handedness(&self) -> bool{
        self.transform_swap_handedness
    }
}
pub trait BaseShapeAble {
     fn object_bound(&self) -> Bounds3;
    fn object_world_bound(&self) -> Bounds3;
    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction>;
    fn intersect_p(&self, ray: &Ray) -> Option<SurfaceInteraction>;
    fn area(&self) -> f32;
    //采样交点，并返回相应的 交互信息与pdf
    fn sample(&self, u: &Point2) -> (Interaction,f32);
    fn pdf(&self, interaction: &Interaction) -> f32 {
        1.0 / self.area()
    }
    fn pdf_iter(&self, interaction: &Interaction, wi: &Vec3) -> f32;
    fn sample_inter(&self, interaction: &Interaction, u: &Point2) -> (Interaction,f32) {
        self.sample(u)
    }
    fn obj_to_world(&self) ->Mat4;
    fn world_to_object(&self) -> Mat4;
    fn reverse_orientation(&self) -> bool;
    //手性交换
    fn transform_swap_handedness(&self) -> bool;
}
