use std::sync::Arc;

use rand::{thread_rng, Rng};

use crate::core::scene::{self, Scene};

use super::{Light, light};

pub struct UnifomrStrategy{
    pub rand: rand::ThreadRng,
}
unsafe impl Sync for UnifomrStrategy{

}
impl<'a> UnifomrStrategy{
    pub fn get_light(&mut self,scene:&'a Scene)->Option<&'a Arc<Light>>{
        // rand::distributions::range::SampleRange::sample_range(r, rng)
        let i=self.rand.gen_range(0, scene.lights.len());
        scene.lights.get(i)
    }
    pub fn new()->Self{
        let rand=thread_rng();
        Self { rand }
    }
}
pub struct PowerStrategy{

}
pub struct SpatialStrategy{

}
pub struct Distribution1D {
    pub func: Vec<f64>,
    pub cdf: Vec<f64>,
    pub func_int: f64,
}
