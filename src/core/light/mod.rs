use crate::{
    extends::{Point2, Vector3},
    until::transform::Transforms,
};

use super::{interaction::Interaction, medium::Medium, spectrum::RGBSpectrum, scene::Scene, sample::Sampler};

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
pub struct AreaLight {
    //指示光源的基本类型
    pub flags: LightType,
    //描述光源内部的介质
    pub medium: Option<Medium>,
    //与世界坐标系的关系
    light_to_world: Transforms,
    //用来区域光源，用于光源采样
    nsample: i32,
}
impl AreaLight {
    pub fn new() {}
    pub fn sample_li() {}
    pub fn power() {}
    pub fn preprocess() {}
    pub fn pdf_li() {}
    pub fn le() {}
    pub fn sample_le() {}
    pub fn pdf_le() {}
}
pub enum Light {
    AreaLight(Box<AreaLight>),
}
impl Light {
    pub fn new() {}
    pub fn sample_li(
        &self,
        interaction: &Interaction,
        u: &Point2,
        wi: &Vector3,
        pdf: &mut f64,
        vis: bool,
    ) {
        match self {
            Self::AreaLight(ref v) => AreaLight::sample_li(),
        }
    }
    pub fn power() {}
    pub fn preprocess() {}
    pub fn pdf_li() {}
    pub fn le() {}
    pub fn sample_le() {}
    pub fn pdf_le() {}
}

pub struct VisibilityTester {
    p0: Interaction,
    p1: Interaction,
}
impl VisibilityTester {
    #[inline]
    pub fn new(p0: Interaction, p1: Interaction) -> Self {
        Self { p0: p0, p1: p1 }
    }
    pub fn get_po(&self)->&Interaction{
        &self.p0
    }
    pub fn get_p1(&self)->&Interaction{
        &self.p1
    }
    //确定两点之间的可见性
    pub fn unoccluded(scene:&Scene)->bool{
        todo!()
    }
    pub fn tr(scene:&Scene,sampler:&Sampler)->RGBSpectrum{
        todo!()
    }

}
