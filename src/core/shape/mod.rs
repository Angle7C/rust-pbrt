use super::{aabb::Bounds3, ray::Ray};

pub mod sphere;
pub enum Shape{
    Sphere(sphere::Sphere),
}
impl Shape{
    pub fn object_bound(&self)->Bounds3{
        match  self {
            Self::Sphere(ref sphere)=>{
                sphere.object_bound()
            }
        }
    }
    pub fn object_world_bound (&self)->Bounds3{
        match  self {
            Self::Sphere(ref sphere)=>{
                sphere.world_bound()
            }
        }
    }
    pub fn intersect_p(&self, ray: &Ray)->bool{
        match  self {
            Self::Sphere(t)=>{
                t.intersect_p(ray)
            }
        }
    }

}