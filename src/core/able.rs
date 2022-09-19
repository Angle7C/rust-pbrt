use super::{aabb::Bounds,};
use crate::{extends::*};
use bvh::ray::Ray;
pub trait Hittable {
    //获取包围盒

    fn get_box(&self);
    //求交
    fn hit(ray: &Ray, t_min: f32, t_max: f32) -> bool;
}
pub fn lerp(t: f32, min: f32, max: f32) -> f32 {
    return min * (1.0 - t) + max * t;
}
pub fn affine_normal_none_scale(affine: &Affine3, normal: Normal) -> Normal {
    let (scale, _, _) = affine.to_scale_rotation_translation();
    if scale.x == scale.y && scale.y == scale.z {
        affine.transform_vector3a(normal)
    } else {
        let mut mat = affine.matrix3;
        mat.x_axis = mat.x_axis / scale.x / scale.x;
        mat.y_axis = mat.y_axis / scale.y / scale.y;
        mat.z_axis = mat.z_axis / scale.z / scale.z;
        mat.mul_vec3a(normal)
    }
}
pub fn affine_bound(affine: &Affine3, bound: &Bounds) -> Bounds {
    let min = affine.transform_point3a(bound.min);
    let mut refs = Bounds::init_point(min);
    for i in 1..8 {
        refs = refs.union_point(affine.transform_point3a(bound.rang_point(i)));
    }
    refs
}
pub fn judge_hander(affine: &Affine3) -> bool {
    //判断是不是左手坐标系
    affine.matrix3.determinant().signum() > 0.0
}
