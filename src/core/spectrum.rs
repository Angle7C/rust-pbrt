use std::ops::{Add, Div, Mul, Neg, Sub};
enum SpectrumType {
    RGB,
}
#[derive(Clone, Copy,Debug)]
pub struct RGBSpectrum {
    pub rgb: [f64; 3],
}
impl Default for RGBSpectrum{
    fn default() -> Self {
        Self { rgb: ([0.0,0.0,0.0]) }
    }
}
impl RGBSpectrum {
    pub const MAX: RGBSpectrum = Self {
        rgb: [255.0, 255.0, 255.0],
    };
    pub const ZERO: RGBSpectrum = Self {
        rgb: [0.0, 0.0, 0.0],
    };
    pub const RED: RGBSpectrum = Self {
        rgb: [255.0, 0.0, 0.0],
    };
    pub const GREEN: RGBSpectrum = Self {
        rgb: [0.0, 255.0, 0.0],
    };
    pub const BLUE: RGBSpectrum = Self {
        rgb: [0.0, 0.0, 255.0],
    };
    pub fn init(&self, _v: f64) {
        todo!()
    }
    fn from_rgb(_rgb: [f64; 3], _types: SpectrumType) -> RGBSpectrum {
        todo!()
    }
    pub fn to_rgb(&self) -> image::Rgb<u8> {
        let r = (self.rgb[0].sqrt()).clamp(0.0, 1.0) *255.0;
        let g = (self.rgb[1].sqrt()).clamp(0.0, 1.0) *255.0;
        let b = (self.rgb[2].sqrt()).clamp(0.0, 1.0) *255.0;
     
        image::Rgb([r as u8, g as u8 , b as u8])
    }
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { rgb: ([r, g, b]) }
    }
    
    pub fn from_value(v: f64) -> Self {
        Self::new(v, v, v)
    }
    pub fn sqrt(&self) -> Self {
        let r = self.rgb[0].sqrt();
        let g = self.rgb[0].sqrt();
        let b = self.rgb[0].sqrt();
        Self::new(r, g, b)
    }
    pub fn y(&self) -> f64 {
        let y_weight: [f64; 3] = [0.212_671, 0.715_160, 0.072_169];
        y_weight[0] * self.rgb[0] + y_weight[1] * self.rgb[1] + y_weight[2] * self.rgb[2]
    }
}
impl Mul<Self> for RGBSpectrum {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut rgb = [0.0, 0.0, 0.0];
        for i in 0..3 {
            rgb[i] = self.rgb[i] * rhs.rgb[i];
        }
        Self::from_rgb(rgb, SpectrumType::RGB)
    }
}
impl Div<Self> for RGBSpectrum {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let mut rgb = [0.0, 0.0, 0.0];
        for i in 0..3 {
            rgb[i] = self.rgb[i] / rhs.rgb[i];
        }
        Self::from_rgb(rgb, SpectrumType::RGB)
    }
}
impl Mul<f64> for RGBSpectrum {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let mut rgb = [0.0, 0.0, 0.0];
        for i in 0..3 {
            rgb[i] = self.rgb[i] * rhs;
        }
        Self::from_rgb(rgb, SpectrumType::RGB)
    }
}
impl Div<f64> for RGBSpectrum {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let mut rgb = [0.0, 0.0, 0.0];
        for i in 0..3 {
            rgb[i] = self.rgb[i] / rhs;
        }
        Self::from_rgb(rgb, SpectrumType::RGB)
    }
}
impl Add<Self> for RGBSpectrum {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut rgb = [0.0, 0.0, 0.0];
        for i in 0..3 {
            rgb[i] = self.rgb[i] * rhs.rgb[i];
        }
        Self::from_rgb(rgb, SpectrumType::RGB)
    }
}
impl Sub<Self> for RGBSpectrum {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut rgb = [0.0, 0.0, 0.0];
        for i in 0..3 {
            rgb[i] = self.rgb[i] - rhs.rgb[i];
        }
        Self::from_rgb(rgb, SpectrumType::RGB)
    }
}
impl Neg for RGBSpectrum {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut rgb = [0.0, 0.0, 0.0];
        for i in 0..3 {
            rgb[i] = -self.rgb[i];
        }
        Self::from_rgb(rgb, SpectrumType::RGB)
    }
}
