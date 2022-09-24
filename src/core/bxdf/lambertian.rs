use std::f32::consts::PI;

use crate::core::spectrum::RGBSpectrum;

use super::{BaseBXDF, BxDFable, BxDFtype};

pub struct Lambertian{
    bxdf:BaseBXDF,
    r:RGBSpectrum,
}
impl Lambertian{
    pub fn new(r:RGBSpectrum)->Self{
        let t=BaseBXDF::new(BxDFtype::Diffuse);
        Self{
            bxdf:t,
            r:r
        }
    }
}
impl BxDFable for Lambertian{
    fn f(&self,wo:&crate::extends::Vec3,wi:&crate::extends::Vec3)->RGBSpectrum {
        self.r*(1.0/PI)
    }
    fn get_type(&self)->super::BxDFtype {
        self.bxdf.get_type()
    }
    fn pdf(&self,wi:&crate::extends::Vec3,wo:&crate::extends::Vec3)->f32 {
        todo!()
    }
    fn rho(&self,n_samples:i32,sample1:&crate::extends::Point2,sample2:&crate::extends::Point2)->RGBSpectrum {
        self.r
    }
    fn rho_wo(&self,wo:&crate::extends::Vec3,n_samples:i32,sample:&crate::extends::Point2)->RGBSpectrum {
        self.r
    }
    fn sample_f(&self,wo:&crate::extends::Vec3,wi:&mut crate::extends::Vec3,sample:&crate::extends::Point2,pdf:&mut f32,types :super::BxDFtype)->RGBSpectrum {
        todo!()
    }
}