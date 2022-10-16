use cgmath::Zero;

use crate::extends::Vector3;

use super::{
    aabb::Bounds3,
    interaction::SurfaceInteraction,
    light::Light,
    primitives::{Aggregate},
    ray::Ray,
    shape::Shape,
};

pub struct Scene<'a> {
    //图元的原始数据
    pub shapes: Vec<Shape>,
    pub lights: Vec<Light>,
    pub aggregate: Aggregate<'a>,
    world_bound: Bounds3,
}
impl<'a> Default for Scene<'a> {
    fn default() -> Self {
        Self {
            shapes: vec![],
            lights: vec![],
            aggregate: Aggregate::default(),
            world_bound: Bounds3::default(),
        }
    }
}
impl<'a> Scene<'a> {
    /// 利用BVH对场景中的物体求交。
    pub fn intersect(&self, ray: &Ray, isect: &mut SurfaceInteraction) -> bool {
        assert_ne!(ray.d, Vector3::zero());
        // self.primitives.
        self.aggregate.hit(ray, isect)
    }
    pub fn intersect_p(&self, _ray: &mut Ray) -> bool {
        unimplemented!()
    }
    pub fn new() -> Self {
        unimplemented!()
    }
    pub fn add_shape(&mut self) {
        unimplemented!()
    }
    pub fn add_light(&mut self) {
        unimplemented!()
    }
    pub fn world_bound(&self) -> &Bounds3 {
        &self.world_bound
    }
}
