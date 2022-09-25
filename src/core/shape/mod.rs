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
    //本地坐标与世界坐标的变换
    pub obj_to_world: Transform,
    //法线方向是不是反向
    pub reverse_orientation: bool,
    //本地坐标系是不是交换了手性
    pub transform_swap_handedness: bool,
}
impl BaseShape {
    ///构造方法
    pub fn new(object_to_world: Mat4, reverse_orientation: bool) -> Self {
        Self {
            obj_to_world: (Transform::new(object_to_world)),
            reverse_orientation: (reverse_orientation),
            transform_swap_handedness: (false),
        }
    }
    ///变换矩阵
    pub fn obj_to_world(&self) -> Transform{
        self.obj_to_world
    }
    ///法线反向
    pub fn reverse_orientation(&self) -> bool{
        self.reverse_orientation
    }
    ///手性交换
    pub fn transform_swap_handedness(&self) -> bool{
        self.transform_swap_handedness
    }
}
pub trait BaseShapeAble {
    ///获得物体包围盒
    fn object_bound(&self) -> Bounds3;
    ///获得物体在世界的包围盒
    fn object_world_bound(&self) -> Bounds3{
        let bound=self.object_bound();
        self.obj_to_world().applying_box_3(&bound)
    }
    ///求是否有交点
    fn intersect(&self, ray: &Ray) -> Option<Interaction>;
    ///求是否有交点,并携带差分信息
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
    //手性变换
    fn reverse_orientation(&self) -> bool;
    //手性交换
    fn transform_swap_handedness(&self) -> bool;
}
