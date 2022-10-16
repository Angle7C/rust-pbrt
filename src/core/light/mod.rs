use crate::{
    extends::{Point2, Vector3},
};

use self::pointlight::PointLight;

use super::{
    interaction::SurfaceInteraction,
    spectrum::RGBSpectrum,
};

pub mod light;
pub mod pointlight;
pub enum LightType {
    DeltaPosition = 1,
    DeltaDirection = 2,
    Area = 4,
    Infinite = 8,
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
//光源的强度分布。
pub enum LightDistribution {}
impl Light {
    pub fn new() {}
    pub fn sample_li(
        &self,
        _interaction: &SurfaceInteraction,
        _u: &Point2,
        _wi: &mut Vector3,
        _pdf: &mut f64,
        // vis: &mut VisibilityTester,
    ) -> RGBSpectrum {
        unimplemented!()
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
