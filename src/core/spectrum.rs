use std::ops::{Add, Div, Mul, Neg, Sub};
enum SpectrumType {
    Reflectance,
    Illuminant,
    RGB,
}
#[derive(Clone, Copy)]
pub struct RGBSpectrum {
    pub rgb: [f32; 3],
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
    fn init(&self, v: f32) {
        todo!()
    }
    fn from_rgb(rgb: [f32; 3], types: SpectrumType) -> RGBSpectrum {
        todo!()
    }
    pub fn to_rgb(&self) -> image::Rgb<u8> {
        let r = self.rgb[0].clamp(0.0, 255.0) as u8;
        let g = self.rgb[1].clamp(0.0, 255.0) as u8;
        let b = self.rgb[2].clamp(0.0, 255.0) as u8;
        image::Rgb([r, g, b])
    }
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { rgb: ([r, g, b]) }
    }
    pub fn from_value(v: f32) -> Self {
        Self::new(v, v, v)
    }
    pub fn sqrt(&self) -> Self {
        let r = self.rgb[0].sqrt();
        let g = self.rgb[0].sqrt();
        let b = self.rgb[0].sqrt();
        Self::new(r, g, b)
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
impl Mul<f32> for RGBSpectrum {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut rgb = [0.0, 0.0, 0.0];
        for i in 0..3 {
            rgb[i] = self.rgb[i] * rhs;
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
