use super::spectrum::RGBSpectrum;
use crate::extends::{Point2, Vec3};
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

    Nil,
}
pub trait BxDFable {
    //返回给定方向的分布函数。
    fn f(&self, wo: &Vec3, wi: &Vec3) -> RGBSpectrum;
    //返回给定方向与采样的分布函数。
    fn sample_f(
        &self,
        wo: &Vec3,
        wi: &mut Vec3,
        sample: &Point2,
        pdf: &mut f32,
        types: BxDFtype,
    ) -> RGBSpectrum;
    //计算半球折射率
    fn rho_wo(&self, wo: &Vec3, n_samples: i32, sample: &Point2) -> RGBSpectrum;
    //计算折射率
    fn rho(&self, n_samples: i32, sample1: &Point2, sample2: &Point2) -> RGBSpectrum;
    //计算pdf；
    fn pdf(&self, wi: &Vec3, wo: &Vec3) -> f32;

    fn get_type(&self) -> BxDFtype;
}
pub struct BaseBXDF {
    pub typs: BxDFtype,
}
impl BaseBXDF {
    pub fn match_type(&self, t: BxDFtype) -> bool {
        (self.typs.to_number() | t.to_number()) > 0
    }
    pub fn new(t: BxDFtype) -> Self {
        Self { typs: (t) }
    }
}

impl BxDFtype {
    fn to_number(&self) -> i32 {
        match self {
            BxDFtype::Reflection => 1,
            BxDFtype::Transmission => 1 << 1,
            BxDFtype::Diffuse => 1 << 2,
            BxDFtype::Glossy => 1 << 3,
            BxDFtype::Specular => 1 << 4,
            BxDFtype::All=> (1 | 2 | 4 | 8 | 16),
            _ => 0,
        }
    }
}

pub struct ScaleBxDF {
    pub base: BaseBXDF,
    bxdf: Box<dyn BxDFable>,
    scale: RGBSpectrum,
}
impl ScaleBxDF {
    pub fn init() -> Self {
        Self {
            base: (BaseBXDF::new(BxDFtype::Nil)),
            bxdf: (Box::new(lambertian::Lambertian::init())),
            scale: (RGBSpectrum::ZERO),
        }
    }
    pub fn new(bxdf: Box<dyn BxDFable>, scale: RGBSpectrum) -> Self {
        let base = BaseBXDF::new(bxdf.get_type());
        Self {
            scale: scale,
            base: base,
            bxdf: bxdf,
        }
    }
}
impl BxDFable for ScaleBxDF {
    fn f(&self, _wo: &Vec3, _wi: &Vec3) -> RGBSpectrum {
        todo!();
    }
    fn get_type(&self) -> BxDFtype {
        self.base.typs
    }
    fn pdf(&self, _wi: &Vec3, _wo: &Vec3) -> f32 {
        todo!()
    }
    fn rho(&self, _n_samples: i32, _sample1: &Point2, _sample2: &Point2) -> RGBSpectrum {
        // self.scale*self.bxdf.rho(n_samples, sample1, sample2)
        todo!()
    }
    fn rho_wo(&self, _wo: &Vec3, _n_samples: i32, _sample: &Point2) -> RGBSpectrum {
        // self.scale*self.bxdf.rho_wo(wo, n_samples, sample)
        todo!()
    }
    fn sample_f(
        &self,
        _wo: &Vec3,
        _wi: &mut Vec3,
        _sample: &Point2,
        _pdf: &mut f32,
        _types: BxDFtype,
    ) -> RGBSpectrum {
        todo!()
    }
}
