use std::{f32::consts::PI, rc::Rc};
pub mod sphere;
use crate::{
    core::{
        aabb::Bounds3,
        interaction::{Interaction, SurfaceInteraction},
        ray::Ray,
    },
    extends::{Affine3, Mat4, Point2, Vec3}, until::transform::Transform,
};
//对线编译器。
pub struct BaseShape {
    pub obj_to_world: Transform,
    pub reverse_orientation: bool,
    pub transform_swap_handedness: bool,
}
impl BaseShape {
    pub fn new(object_to_world: Mat4, reverse_orientation: bool) -> Self {
        Self {
            obj_to_world: (Transform::new(object_to_world)),
            reverse_orientation: (reverse_orientation),
            transform_swap_handedness: (false),
        }
    }
    pub fn obj_to_world(&self) -> Transform{
        self.obj_to_world
    }
    pub fn reverse_orientation(&self) -> bool{
        self.reverse_orientation
    }
    //手性交换
    pub fn transform_swap_handedness(&self) -> bool{
        self.transform_swap_handedness
    }
}
pub trait BaseShapeAble {
    ///获得物体包围盒
    fn object_bound(&self) -> Bounds3;
    ///获得物体在世界的包围盒
    fn object_world_bound(&self) -> Bounds3;
    ///求是否有交点
    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction>;
    ///求是否有交点
    fn intersect_p(&self, ray: &Ray) -> Option<SurfaceInteraction>;
    /// 求表面积
    fn area(&self) -> f32;
    ///采样交点，并返回相应的 交互信息与pdf
    fn sample(&self, u: &Point2) -> (Interaction,f32);
    ///返回对应的 概率分布
    fn pdf(&self, interaction: &Interaction) -> f32 {
        1.0 / self.area()
    }
    ///返回对应的 概率分布
    fn pdf_iter(&self, interaction: &Interaction, wi: &Vec3) -> f32;
    fn sample_inter(&self, interaction: &Interaction, u: &Point2) -> (Interaction,f32) {
        self.sample(u)
    }
    //获取相应的变换矩阵
    fn obj_to_world(&self) ->Transform;
    //
    fn reverse_orientation(&self) -> bool;
    //手性交换
    fn transform_swap_handedness(&self) -> bool;
}
