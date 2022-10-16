
use crate::{
    core::spectrum::RGBSpectrum,
};

use super::BxdfType;

pub enum Fresnel {
    NoOp(FresnelNoOp),
    Conductor(FresnelConductor),
    Dielectric(FresnelDielectric),
    // Disney(DisneyFresnel),
}
impl Fresnel {
    pub fn evaluate(&self, cos_theta_i: f64) -> RGBSpectrum {
        match self {
            Self::NoOp(v) => v.evaluate(cos_theta_i),
            Self::Conductor(v) => v.evaluate(cos_theta_i),
            Self::Dielectric(v) => v.evaluate(cos_theta_i),
        }
    }
}
pub struct FresnelConductor {
    pub eta_i: RGBSpectrum,
    pub eta_t: RGBSpectrum,
    pub k: RGBSpectrum,
}

impl FresnelConductor {
    pub fn evaluate(&self, cos_theta_i: f64) -> RGBSpectrum {
        fr_conductor(cos_theta_i, self.eta_i, self.eta_t, self.k)
    }
}
pub struct FresnelDielectric {
    pub eta_i: f64,
    pub eta_t: f64,
}
impl FresnelDielectric {
    pub fn evaluate(&self, cos_theta_i: f64) -> RGBSpectrum {
        RGBSpectrum::from_value(fr_dielectric(cos_theta_i, self.eta_i, self.eta_t))
    }
    pub fn new(eta_i: f64, eta_t: f64) -> Self {
        Self { eta_i, eta_t }
    }
}
pub struct FresnelNoOp {}
impl FresnelNoOp {
    pub fn evaluate(&self, _cos_theta_i: f64) -> RGBSpectrum {
        RGBSpectrum::from_value(1.0)
    }
}
pub fn fr_conductor(
    cos_theta_i: f64,
    eta_i: RGBSpectrum,
    eta_t: RGBSpectrum,
    k: RGBSpectrum,
) -> RGBSpectrum {
    let not_clamped: f64 = cos_theta_i;
    let cos_theta_i: f64 = f64::clamp(not_clamped, -1.0, 1.0);
    let eta: RGBSpectrum = eta_t / eta_i;
    let eta_k: RGBSpectrum = k / eta_i;
    let cos_theta_i2: f64 = cos_theta_i * cos_theta_i;
    let sin_theta_i2: f64 = 1.0 as f64 - cos_theta_i2;
    let eta_2: RGBSpectrum = eta * eta;
    let eta_k2: RGBSpectrum = eta_k * eta_k;
    let t0: RGBSpectrum = eta_2 - eta_k2 - RGBSpectrum::from_value(sin_theta_i2);
    let a2_plus_b2: RGBSpectrum =
        (t0 * t0 + eta_2 * eta_k2 * RGBSpectrum::from_value(4.0 as f64)).sqrt();
    let t1: RGBSpectrum = a2_plus_b2 + RGBSpectrum::from_value(cos_theta_i2);
    let a: RGBSpectrum = ((a2_plus_b2 + t0) * 0.5 as f64).sqrt();
    let t2: RGBSpectrum = a * 2.0 as f64 * cos_theta_i;
    let rs: RGBSpectrum = (t1 - t2) / (t1 + t2);
    let t3: RGBSpectrum =
        a2_plus_b2 * cos_theta_i2 + RGBSpectrum::from_value(sin_theta_i2 * sin_theta_i2);
    let t4: RGBSpectrum = t2 * sin_theta_i2;
    let rp: RGBSpectrum = rs * (t3 - t4) / (t3 + t4);
    (rp + rs) * RGBSpectrum::from_value(0.5 as f64)
}
pub fn fr_dielectric(cos_theta_i: f64, eta_i: f64, eta_t: f64) -> f64 {
    let mut cos_theta_i = f64::clamp(cos_theta_i, -1.0, 1.0);
    let entering: bool = cos_theta_i > 0.0;
    let mut local_eta_i = eta_i;
    let mut local_eta_t = eta_t;
    if !entering {
        std::mem::swap(&mut local_eta_i, &mut local_eta_t);
        cos_theta_i = cos_theta_i.abs();
    }
    let sin_theta_i: f64 = (0.0 as f64)
        .max(1.0 as f64 - cos_theta_i * cos_theta_i)
        .sqrt();
    let sin_theta_t: f64 = local_eta_i / local_eta_t * sin_theta_i;
    if sin_theta_t >= 1.0 as f64 {
        return 1.0 as f64;
    }
    let cos_theta_t: f64 = (0.0 as f64)
        .max(1.0 as f64 - sin_theta_t * sin_theta_t)
        .sqrt();
    let r_parl: f64 = ((local_eta_t * cos_theta_i) - (local_eta_i * cos_theta_t))
        / ((local_eta_t * cos_theta_i) + (local_eta_i * cos_theta_t));
    let r_perp: f64 = ((local_eta_i * cos_theta_i) - (local_eta_t * cos_theta_t))
        / ((local_eta_i * cos_theta_i) + (local_eta_t * cos_theta_t));
    (r_parl * r_parl + r_perp * r_perp) / 2.0
}

/// Specular Relection
pub struct SpecularReflection {
    pub r: RGBSpectrum,
    pub fresnel: Fresnel,
    pub bxdf_type: BxdfType,
}
impl SpecularReflection {
    pub fn new(r: RGBSpectrum, fresnel: Fresnel, bxdf_type: BxdfType) -> Self {
        Self {
            r,
            fresnel,
            bxdf_type,
        }
    }
}
