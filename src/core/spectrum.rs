enum SpectrumType{
    Reflectance,
    Illuminant,
}
pub struct RGBSpectrum{
    pub RGB:[f32;3],
}
impl RGBSpectrum {
    fn init(&self,v:f32){
        todo!()
    }
    fn from_rgb(rgb:[f32;3],types :SpectrumType)->RGBSpectrum{
        todo!()
    }
    fn to_rgb(&self)->[f32;3]{
        todo!()
    }
    fn to_RGB_spectrum(&self)->&Self{
        todo!();
    }
}