use super::aabb::Bounds3;

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

}