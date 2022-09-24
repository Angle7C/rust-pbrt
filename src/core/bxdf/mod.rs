
use crate::extends::{Vec3, Point2};
use super::spectrum::RGBSpectrum;
pub mod lambertian;
pub mod reflection;
#[derive(Clone, Copy)]
pub enum BxDFtype {
    //反射
    Reflection,
    //传输
    Transmission,
    //漫反射
    Diffuse,
    //光泽
    Glossy,
    //高光
    Specular,
    
    All,
    
    Nil
}
pub trait BxDFable {
    //返回给定方向的分布函数。
    fn f(&self,wo:&Vec3,wi:&Vec3)->RGBSpectrum;
    //返回给定方向与采样的分布函数。
    fn sample_f(&self,wo:&Vec3,wi:&mut Vec3,sample:&Point2,pdf:&mut f32,types :BxDFtype)->RGBSpectrum;
    //计算半球折射率
    fn rho_wo(&self,wo:&Vec3,n_samples:i32,sample:&Point2)->RGBSpectrum;
    //计算折射率
    fn rho(&self,n_samples:i32,sample1:&Point2,sample2:&Point2)->RGBSpectrum;
    //计算pdf；
    fn pdf(&self,wi:&Vec3,wo:&Vec3)->f32;

    fn get_type(&self)->BxDFtype;
}
pub struct  BaseBXDF{
    pub typs :BxDFtype,
}
impl BaseBXDF{
    pub fn match_type(&self,t:BxDFtype)->bool{
       (self.typs.to_number()|t.to_number())>0
    }
    pub fn new(t:BxDFtype)->Self{
        Self { typs: (t) }
    }
}
impl BxDFable for BaseBXDF{
    fn f(&self,wo:&Vec3,wi:&Vec3)->RGBSpectrum {
        todo!()
    }
    fn pdf(&self,wi:&Vec3,wo:&Vec3)->f32 {
        todo!()
    }
    fn rho(&self,n_samples:i32,sample1:&Point2,sample2:&Point2)->RGBSpectrum {
        todo!()
    }
    fn rho_wo(&self,wo:&Vec3,n_samples:i32,sample:&Point2)->RGBSpectrum {
        todo!()
    }
    fn sample_f(&self,wo:&Vec3,wi:&mut Vec3,sample:&Point2,pdf:&mut f32,types :BxDFtype)->RGBSpectrum {
        todo!()
    }
    fn get_type(&self)->BxDFtype {
        self.typs
    }
}
impl  BxDFtype {
    fn to_number(&self)-> i32 {
        match self {
            BxDFtype::Reflection=>1,
            BxDFtype::Transmission=>1<<1,           
            BxDFtype::Diffuse=>1<<2,
            BxDFtype::Glossy=>1<<3,
            BxDFtype::Specular=>1<<4,
            BxDFtype::Specular=> (1|2|4|8|16),
            _=>0,
        }
    }
}

pub struct ScaleBxDF{
    pub base :BaseBXDF,
    bxdf :Box<dyn BxDFable>,
    scale:RGBSpectrum,
}
impl ScaleBxDF{
    pub fn new<'a>(bxdf:Box<dyn BxDFable>,scale:RGBSpectrum)->Self{
        let base=BaseBXDF::new(bxdf.get_type());
        Self{
            scale:scale,
            base:base,
            bxdf:bxdf,
        }
    }
}
impl BxDFable for ScaleBxDF{
    fn f(&self,wo:&Vec3,wi:&Vec3)->RGBSpectrum {
        todo!();   
    }
    fn get_type(&self)->BxDFtype {
        self.base.typs
    }
    fn pdf(&self,wi:&Vec3,wo:&Vec3)->f32 {
        todo!()
    }
    fn rho(&self,n_samples:i32,sample1:&Point2,sample2:&Point2)->RGBSpectrum {
        // self.scale*self.bxdf.rho(n_samples, sample1, sample2)
        todo!()
    }
    fn rho_wo(&self,wo:&Vec3,n_samples:i32,sample:&Point2)->RGBSpectrum {
        // self.scale*self.bxdf.rho_wo(wo, n_samples, sample)
        todo!()
    }
    fn sample_f(&self,wo:&Vec3,wi:&mut Vec3,sample:&Point2,pdf:&mut f32,types :BxDFtype)->RGBSpectrum {
        todo!()
    }

}