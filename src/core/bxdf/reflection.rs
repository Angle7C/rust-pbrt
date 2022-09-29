use std::{ mem::swap};

use crate::{core::spectrum::RGBSpectrum, extends::*};
//计算菲涅尔
pub trait FresnelAble {
    fn evaluate(&mut self, wi: &Vec3, normal: &Normal) -> RGBSpectrum;
}
//导体与电介质菲涅尔
pub struct FresnelConductor {

    eta_i: RGBSpectrum,
    //折射率
    eta_t: RGBSpectrum,
    //投射率
    k: RGBSpectrum,
}
impl FresnelAble for FresnelConductor {
    fn evaluate(&mut self, wi: &Vec3, normal: &Normal) -> RGBSpectrum {
        let cos = wi.dot(*normal).abs().clamp(-1.0, 1.0);
        let n = self.eta_t / self.eta_i;
        let k = self.k / self.eta_i;
        let cos_i2 = cos * cos;
        let sin_i2 = 1.0 - cos_i2;
        let n2 = n * n;
        let k2 = k * k;
        let t0 = n2 - k2 - RGBSpectrum::from_value(sin_i2);
        let a_and_b2=t0*t0+n2*k2*4.0;
        let t1=a_and_b2+RGBSpectrum::from_value(cos_i2);
        let a=((a_and_b2+t0)*0.5).sqrt();
        let t2=a*cos*2.0;
        let rs=(t1-t2)/(t1+t2);
        let t3=a_and_b2*cos_i2+RGBSpectrum::from_value(sin_i2*sin_i2);
        let t4=t2*sin_i2;
        let rp=rs*(t3-t4)/(t3+t4);
        (rp+rs)*0.5
    }
}
//电介质菲涅尔
pub struct FresnelDielectric {
    eta_i: f32,
    eta_t: f32,
}
impl FresnelAble for FresnelDielectric {
    fn evaluate(&mut self, wi: &Vec3, normal: &Normal) -> RGBSpectrum {
        let mut cos_i = wi.dot(*normal).clamp(-1.0, 1.0);
        if cos_i > 0.0 {
            swap(&mut self.eta_i, &mut self.eta_t);
            cos_i = cos_i.abs();
        };
        let sin_i = f32::max(0.0, 1.0 - cos_i * cos_i).sqrt();
        let sin_t = self.eta_i / self.eta_t * sin_i;
        if sin_i >= 1.0 {
            return RGBSpectrum::MAX;
        };
        let cos_t = f32::max(0.0, 1.0 - sin_t * sin_t).sqrt();
        let det = self.eta_t * cos_i + self.eta_i * cos_t;
        let a = (self.eta_t * cos_i - self.eta_i * cos_t) / det;
        let b = (self.eta_i * cos_i - self.eta_t * cos_t) / det;
        RGBSpectrum::from_value((a * a + b * b) / 2.0)
    }
}
//无折射菲涅尔
pub struct FresnelNoOp;
impl FresnelAble for FresnelNoOp{
    fn evaluate(&mut self, _wi: &Vec3, _normal: &Normal) -> RGBSpectrum {
        RGBSpectrum::from_value(1.0)
    }
}
