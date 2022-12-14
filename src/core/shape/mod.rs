use super::{aabb::Bounds3, ray::Ray, interaction::SurfaceInteraction};

pub mod sphere;
#[derive(Debug)]
pub enum Shape{
    Sphere(sphere::Sphere),
}
impl<'a> Shape{
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
    pub fn intersect(&self, ray: &Ray,isect:&mut SurfaceInteraction<'a>)->bool{
     
            match self {
                Self::Sphere(v)=>{
                    v.intersect(ray,isect)
                },
            }
    }
    pub fn reverse_orientation(&self)->bool{
        match self {
            Self::Sphere(v)=>v.reverse_orientation
        }
    }
    pub fn transform_swaps_handedness(&self)->bool{
        match self {
            Self::Sphere(v)=>v.get_transform_swaps_handedness()
        }
    }

}