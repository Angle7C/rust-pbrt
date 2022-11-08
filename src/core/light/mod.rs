
pub mod light;
pub mod pointlight;
pub mod light_strategy;
use crate::{
    extends::{Point2, Vector3},
};

use self::{pointlight::PointLight, light_strategy::UnifomrStrategy};

use super::{
    interaction::{SurfaceInteraction, Interaction},
    spectrum::RGBSpectrum, scene::{self, Scene},
};

pub enum LightType {
    DeltaPosition = 1,
    DeltaDirection = 2,
    Area = 4,
    Infinite = 8,
}
#[derive(Debug,Clone, Copy)]
pub enum TransportMode{
    Radiance,
    Importance
}
impl LightType {
    pub fn is_delta(&self) -> bool {
        match self {
            Self::DeltaDirection | Self::DeltaPosition => true,
            _ => false,
        }
    }
}
pub enum Light {
    Nil,
    PointLight(Box<PointLight>),
}
//光源的采样策略
pub enum LightDistribution {
    Uniform(UnifomrStrategy),

}
unsafe impl Sync for LightDistribution{

}
#[derive(Default)]
pub struct VisibilityTester<'a,'b>{
    pub p0 :Option<&'a Interaction>,
    pub p1 :Option<&'b Interaction>,
}
impl<'a,'b>VisibilityTester<'a,'b>{
    pub fn unocclued(&self,scene:&Scene)->bool{
        let mut ray=self.p0.unwrap().spawn_ray(self.p1.unwrap());
        !scene.intersect_p(&mut ray)
    }
}

impl Light {
    pub fn new() {}
    pub fn sample_li(
        &self,
        interaction: &Interaction,
        light_interaction:&mut Interaction,
        u: &Point2,
        wi: &mut Vector3,
        pdf: &mut f64,
        vis: &mut VisibilityTester,
    ) -> RGBSpectrum {
        match self {
            Self::PointLight(v)=>v.sample_li(&interaction,light_interaction, u, wi, pdf,vis),
            Self::Nil=>unimplemented!(),
        }
    }
    pub fn power() {}
    pub fn preprocess() {}
    pub fn pdf_li() {}
    pub fn le() {}
    pub fn sample_le() {}
    pub fn pdf_le() {}

    pub fn light_to_point(&self) -> bool {
        unimplemented!()
    }
}
