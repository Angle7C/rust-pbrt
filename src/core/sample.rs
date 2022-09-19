use crate::extends::Point2;

pub struct Sample{
    
}
pub trait SampleAble {
    
}
pub struct CameraSample{
    pub sample :Sample,
    pub p_film :Point2,
    pub p_lens:Point2,
    pub time :f32,
}
