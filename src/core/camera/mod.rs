use self::perspecttivecamera::PerspectiveCamera;

pub mod perspecttivecamera;
pub enum Camera{
    Orthographic(),
    Perspective(Box<PerspectiveCamera>),
    
}