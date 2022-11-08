use cgmath::{EuclideanSpace, InnerSpace, Zero};

use super::{
    bssrdf::Bssrdf, light::TransportMode, medium::Medium, primitives::Primitive, ray::Ray,
    reflection::bsdf::BSDF, shape::Shape, spectrum::RGBSpectrum,
};
use crate::extends::{Point2, Point3, Vector3};
#[derive(Debug, Clone)]
pub struct Interaction {
    //击中的点
    pub p: Point3,
    //光线击中时间
    pub time: f64,
    //与入射光线相反
    pub wo: Vector3,
    //该点的法向
    pub normal: Vector3,
    //体积渲染介质
    pub medium: Option<Medium>,
}
impl Default for Interaction {
    fn default() -> Self {
        Self {
            p: Point3::origin(),
            time: 0.0,
            wo: Vector3::zero(),
            normal: Vector3::zero(),
            medium: None,
        }
    }
}
impl Interaction {
    // p 和 t 是必须的
    pub fn new(p: Point3, t: f64, w: Vector3, normal: Vector3, medium: Option<Medium>) -> Self {
        Self {
            p: p,
            time: t,
            wo: w,
            normal: normal.normalize(),
            medium: medium,
        }
    }
    pub fn spawn_ray(&self,it:&Interaction)->Ray{
        let d=it.p-self.p;
        let o=self.p;
        let mut ray=Ray::new(o, d);
        ray.d=d;
        ray.t_max=1.0;
        ray
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Shading {
    pub n: Vector3,
    pub dpdu: Vector3,
    pub dpdv: Vector3,
    pub dndu: Vector3,
    pub dndv: Vector3,
}
#[derive(Clone, Debug)]
pub struct SurfaceInteraction<'a> {
    pub p: Point3,
    //光线击中时间
    pub time: f64,
    //与入射光线相反
    pub wo: Vector3,
    //该点的法向
    pub normal: Vector3,
    //
    pub medium: Option<Medium>,
    pub uv: Point2,
    pub dpdu: Vector3,
    pub dpdv: Vector3,
    pub dndu: Vector3,
    pub dndv: Vector3,
    pub dpdx: Vector3,
    pub dpdy: Vector3,
    pub dudx: f64,
    pub dvdx: f64,
    pub dudy: f64,
    pub dvdy: f64,
    pub primitive: Option<*const Primitive>,
    pub shading: Shading,
    pub bsdf: Option<BSDF>,
    pub bssrdf: Option<Bssrdf>,
    pub shape: Option<&'a Shape>,
}
impl<'a> Default for SurfaceInteraction<'a> {
    fn default() -> Self {
        Self {
            p: Point3::origin(),
            time: 0.0,
            wo: Vector3::zero(),
            normal: Vector3::zero(),
            medium: None,
            uv: Point2::origin(),
            dpdu: Vector3::zero(),
            dpdv: Vector3::zero(),
            dndu: Vector3::zero(),
            dndv: Vector3::zero(),
            dpdx: Vector3::zero(),
            dpdy: Vector3::zero(),
            dudx: 0.0,
            dvdx: 0.0,
            dudy: 0.0,
            dvdy: 0.0,
            primitive: None,
            shading: Shading {
                n: Vector3::zero(),
                dpdu: Vector3::zero(),
                dpdv: Vector3::zero(),
                dndu: Vector3::zero(),
                dndv: Vector3::zero(),
            },
            bsdf: None,
            bssrdf: None,
            shape: None,
        }
    }
}
impl<'a> SurfaceInteraction<'a> {
    pub fn new(p: Point3, time: f64, w0: Vector3, normal: Vector3) -> Self {
        let mut obj = Self::default();
        obj.p = p;
        obj.time = time;
        obj.wo = w0;
        obj.normal = normal;
        obj
    }
    //设置渲染参数
    pub fn set_shading_geometry(
        &mut self,
        dpdu: &Vector3,
        dpdv: &Vector3,
        dndu: &Vector3,
        dndv: &Vector3,
        time: f64,
        shape: &'a Shape,
        is_authoritative: bool,
    ) {
        if let Some(v) = self.shape {
            if v.reverse_orientation() ^ v.transform_swaps_handedness() {
                self.shading.n = -self.shading.n;
            }
            if is_authoritative {
                self.normal = if self.normal.dot(self.shading.n) > 0.0 {
                    self.normal
                } else {
                    -self.normal
                }
            } else {
                self.shading.n = if self.normal.dot(self.shading.n) > 0.0 {
                    self.shading.n
                } else {
                    -self.shading.n
                }
            };
            self.shading.dpdu = *dpdu;
            self.shading.dpdv = *dpdv;
            self.shading.dndu = *dndu;
            self.shading.dndv = *dndv;
        }
    }
    pub fn compute_scattering_functions(
        &mut self,
        ray:&Ray,
        mode: TransportMode,
        multips_lobes: bool,
    ) {
        self.compute_differentials(ray);
        if let Some(v) = self.primitive {
            let p = unsafe { &*v };
            p.compute_scattering_functions(self, mode, multips_lobes);
        }
    }
    pub fn compute_differentials(&self, ray: &Ray) {}
    pub fn le(&self, w: &Vector3) -> RGBSpectrum {
        unimplemented!()
    }
    pub fn spawn_ray(&self, d: Vector3) -> Ray {
        Ray::new(self.p, d)
    }
    pub fn to_interaction(&self)->Interaction{
        Interaction::new(self.p, self.time, self.wo, self.normal, self.medium)
    }

}
