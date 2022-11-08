use std::f64::consts::PI;

use cgmath::{EuclideanSpace, InnerSpace, MetricSpace, Transform};

use crate::{
    core::{
        interaction::Interaction,
        medium::Medium,
        ray::Ray,
        spectrum::{RGBSpectrum},
    },
    extends::{Mat4, Point2, Point3, Vector3},
    until::{transform::Transforms, untils::{uniform_sample_phere, uniform_sample_sphere_pdf}},
};

use super::{LightType, VisibilityTester};

pub struct PointLight {
    //世界坐标下
    p_light: Point3,
    //单位辐射功率
    spectrum: RGBSpectrum,
    pub flags: LightType,
    //描述光源内部的介质
    pub medium: Option<Medium>,
    //与世界坐标系的关系
    #[allow(unused)]
    light_to_world: Transforms,
    //用来区域光源，用于光源采样
    // nsample: i32,
}
impl PointLight {
    pub fn new(light_to_world: Mat4, medium: Option<Medium>, spectrum: RGBSpectrum) -> Self {
        Self {
            p_light: light_to_world.transform_point(Point3::origin()),
            flags: LightType::DeltaPosition,
            spectrum: spectrum,
            light_to_world: Transforms::new(light_to_world),
            medium: medium,
            // nsample: 0,
        }
    }
    //采样点光源
    pub fn sample_li(
        &self,
        interaction: &Interaction,
        light_interaction:&mut Interaction,
        _u: &Point2,
        wi: &mut Vector3,
        pdf: &mut f64,
        vis: &mut VisibilityTester,
    ) -> RGBSpectrum {
        vis.p0=Some(interaction);
        light_interaction.p=self.p_light;
        light_interaction.time=interaction.time;
        vis.p1=Some(light_interaction);
        *wi = (self.p_light - interaction.p).normalize();
        *pdf = 1.0;
        self.spectrum / self.p_light.distance2(interaction.p)
    }
    //点光源的总能量
    pub fn power(&self) -> RGBSpectrum {
        self.spectrum * 4.0 * PI
    }

    pub fn pdf_li(&self, _interaction: &Interaction, _u: &Vector3) -> f64 {
        unimplemented!()
    }
    pub fn sample_le(
        &self,
        u1: &Point2,
        _u2: &Point2,
        time: f64,
        ray: &mut Ray,
        normal: &mut Vector3,
        pdf_pos: &mut f64,
        pdf_dir: &mut f64,
    ) -> RGBSpectrum {
        *ray=Ray::new_all(self.p_light,uniform_sample_phere(u1),f64::INFINITY,time,self.medium,None);
        *normal=ray.d;
        *pdf_pos=1.0;
        *pdf_dir=uniform_sample_sphere_pdf();
        self.spectrum
    }
    pub fn pdf_le(_ray: &Ray, _normal: &Vector3, pdf_pos: &mut f64, pdf: &mut f64) {
        *pdf_pos=0.0;
        *pdf=uniform_sample_sphere_pdf();
    }
}
pub struct SpotLight {
    //世界坐标下
    p_light: Point3,
    //单位辐射功率
    spectrum: RGBSpectrum,
    //falloff-total之间具有过度效果
    cos_total_width: f64,
    //在0-falloff之间完全照亮
    cos_falloff_start: f64,

    pub flags: LightType,
    //描述光源内部的介质
    pub medium: Option<Medium>,
    //与世界坐标系的关系
    #[allow(unused)]
    light_to_world: Transforms,
    //用来区域光源，用于光源采样
        // nsample: i32,
}
impl SpotLight {
    pub fn new(
        light_to_world: Mat4,
        medium: Option<Medium>,
        spectrum: RGBSpectrum,
        total_width: f64,
        falloff_start: f64,
    ) -> Self {
        Self {
            p_light:light_to_world.transform_point(Point3::origin()),
            spectrum: spectrum,
            cos_total_width: total_width.cos(),
            cos_falloff_start: falloff_start.cos(),
            flags: LightType::DeltaPosition,
            medium: medium,
            light_to_world: Transforms::new(light_to_world),
        }

    }
    pub fn sample_li(
        &self,
        interaction: &Interaction,
        _u: &Point2,
        wi: &mut Vector3,
        pdf: &mut f64,

    )->RGBSpectrum{
        *wi = (self.p_light - interaction.p).normalize();
        *pdf = 1.0;
        self.spectrum*self.falloff(&-*wi) / self.p_light.distance2(interaction.p)
    }
    pub fn power(&self)->RGBSpectrum{
        self.spectrum*2.0*PI*(1.0-0.5*(self.cos_total_width+self.cos_total_width))    
    }
    fn falloff(&self,w:&Vector3)->f64{
        let w=w.normalize();
        let cos_theta=w.z;
        if cos_theta<self.cos_total_width{
            return 0.0;   
        };
        if cos_theta>self.cos_falloff_start{
            return 1.0;
        };
        let det=(cos_theta-self.cos_total_width)/(self.cos_falloff_start-self.cos_total_width);
        (det*det)*(det*det)
    }
    
}
