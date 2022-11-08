use std::sync::Arc;

use cgmath::Zero;

use crate::extends::{Point2, Vector3};

use super::{
    aabb::Bounds3,
    interaction::{Interaction, SurfaceInteraction},
    light::{light_strategy::Distribution1D, Light, LightDistribution, VisibilityTester},
    primitives::Aggregate,
    ray::Ray,
    sample::Sampler,
    shape::Shape,
    spectrum::RGBSpectrum,
};

pub struct Scene<'a> {
    //图元的原始数据
    pub shapes: Vec<Shape>,
    pub lights: Vec<Arc<Light>>,
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
    pub fn sample_light(
        &self,
        strategy: LightDistribution,
        isect: &mut SurfaceInteraction,
        u_light: Point2,
        scene: &Scene,
        sampler: &mut Sampler,
        handle_media: bool,
        light_distrib: Distribution1D,
    ) -> RGBSpectrum {
        let mut wi = Vector3::zero();
        let mut light_pdf = 0.0;
        let mut scattering_pdf = 0.0;
        let mut vis = VisibilityTester::default();
        let mut light_interaction = Interaction::default();
        if let Some(light) = match strategy {
            LightDistribution::Uniform(ref v) => v.get_light(self),
            _ => {
                light_pdf = 1.0 / self.lights.len() as f64;
                scene
                    .lights
                    .get((sampler.get_1d() * scene.lights.len() as f64).round() as usize)
            }
        } {
            light.sample_li(
                &isect.to_interaction(),
                &mut light_interaction,
                &u_light,
                &mut wi,
                &mut light_pdf,
                &mut vis,
            );
            if light_pdf == 0.0 {
                return RGBSpectrum::default();
            }
            RGBSpectrum::default()
        } else {
            RGBSpectrum::default()
        }
    }
}
