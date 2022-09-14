use super::observer::Ray;

pub trait Hittable {
    //获取包围盒
    fn get_box(&self);
    //求交
    fn hit(ray :&Ray,t_min:f32,t_max:f32)->bool;
}
