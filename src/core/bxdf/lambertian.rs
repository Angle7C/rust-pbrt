use std::f32::consts::PI;

use crate::core::spectrum::RGBSpectrum;

use super::{BaseBXDF, BxDFable, BxDFtype};

pub struct Lambertian{
    bxdf:BaseBXDF,
    r:RGBSpectrum,
}
impl Lambertian{
    pub fn init()->Self{
        Self::new(RGBSpectrum::ZERO)
    }
    pub fn new(r:RGBSpectrum)->Self{
        let t=BaseBXDF::new(BxDFtype::Diffuse);
        Self{
            bxdf:t,
            r:r
        }
    }
}
impl BxDFable for Lambertian{
    fn f(&self,_wo:&crate::extends::Vec3,_wi:&crate::extends::Vec3)->RGBSpectrum {
        self.r*(1.0/PI)
    }
    fn get_type(&self)->super::BxDFtype {
        self.bxdf.typs
    }
    fn pdf(&self,_wi:&crate::extends::Vec3,_wo:&crate::extends::Vec3)->f32 {
        todo!()
    }
    fn rho(&self,_n_samples:i32,_sample1:&crate::extends::Point2,_sample2:&crate::extends::Point2)->RGBSpectrum {
        self.r
    }
    fn rho_wo(&self,_wo:&crate::extends::Vec3,_n_samples:i32,_sample:&crate::extends::Point2)->RGBSpectrum {
        todo!()
    }
    fn sample_f(&self,_wo:&crate::extends::Vec3,_wi:&mut crate::extends::Vec3,_sample:&crate::extends::Point2,_pdf:&mut f32,_types :super::BxDFtype)->RGBSpectrum {
        todo!()
    }
}