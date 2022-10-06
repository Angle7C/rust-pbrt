use std::ops::BitOr;

use cgmath::InnerSpace;

use crate::extends::{Point2, Vector3};

use self::fresnel::{Fresnel, FresnelDielectric};

use super::spectrum::RGBSpectrum;

pub mod bsdf;
pub mod fresnel;
#[derive(Debug, Clone)]
pub enum BxdfType {
    BsdfReflection = 1,
    BsdfTransmission = 2,
    BsdfDiffuse = 4,
    BsdfGlossy = 8,
    BsdfSpecular = 16,
    BsdfAll = 31,
}
impl BitOr<Self> for BxdfType {
    type Output = u8;
    fn bitor(self, rhs: Self) -> Self::Output {
        self as u8 | rhs as u8
    }
}
#[derive(Debug, Clone)]

pub enum Bxdf {
    SpecularReflection,
}
///BRDF高光反射模型
/// 会将所有的光反射到一个特定的方向，从其他方向观察的值是0.0
/// 即该模型没有散射
pub struct SpecularReflection {
    pub r: RGBSpectrum,
    pub frensel: Fresnel,
    pub types: u8,
}
impl SpecularReflection {
    pub fn new(spectrum: RGBSpectrum, fresnel: Fresnel) -> Self {
        Self {
            r: spectrum,
            frensel: fresnel,
            types: BxdfType::BsdfReflection | BxdfType::BsdfSpecular,
        }
    }
    //计算散射
    #[inline]
    pub fn f(_w0: &Vector3, _wi: &Vector3) -> RGBSpectrum {
        RGBSpectrum::from_value(0.0)
    }
    #[inline]
    //计算折射
    pub fn sample_f(
        &self,
        w0: Vector3,
        wi: &mut Vector3,
        _sample: Point2,
        pdf: &mut f64,
        _bxdf_type: BxdfType,
    ) -> RGBSpectrum {
        *wi = Vector3::new(-w0.x, -w0.y, w0.z);
        *pdf = 1.0;
        self.r * self.frensel.evaluate(wi.z) / wi.z.abs()
    }
    #[inline]
    pub fn pdf(_w0: &Vector3, _wi: &Vector3) -> f64 {
       0.0
    }
    ///反射向量
    #[inline]
    pub fn reflect(w0: &Vector3, n: &Vector3) -> Vector3 {
        -*w0 + (w0.dot(*n)) * n * 2.0
    }
}
pub struct SpecularTransmission {
    pub spectrum: RGBSpectrum,
    //上方折射率
    pub eta_a: f64,
    //下方折射率
    pub eta_b: f64,
    //菲涅尔项
    pub fresnel: Fresnel,

    pub types: u8,
}
impl SpecularTransmission {
    pub fn new(eta_a: f64, eta_b: f64, spectrum: RGBSpectrum) -> Self {
        Self {
            spectrum: spectrum,
            eta_a: eta_a,
            eta_b: eta_b,
            fresnel: Fresnel::Dielectric(FresnelDielectric::new(eta_a, eta_b)),
            types: BxdfType::BsdfTransmission | BxdfType::BsdfSpecular,
        }
    }
    //计算散射
    #[inline]
    pub fn f(_w0: &Vector3, _wi: &Vector3) -> RGBSpectrum {
        RGBSpectrum::from_value(0.0)
        
    }
    
}
