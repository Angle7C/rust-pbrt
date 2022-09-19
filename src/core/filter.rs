use crate::extends::*;
pub struct BaseFilter{
    pub radius :Vec2,   
    pub inv_radius:Vec2,
}
impl BaseFilter{
    pub fn new(radius :&Vec2)->Self{
        Self { radius: (*radius), inv_radius: (Vec2::new(1.0/radius.x,1.0/radius.y)) }
    }
}
pub trait  FilterAble{
    fn get_radius(&self)->Vec2;
    fn get_inv_radius(&self)->Vec2;
    fn evaluate(&self,p:&Point2)->f32;
}