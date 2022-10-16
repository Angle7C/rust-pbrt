use crate::extends::{Point2, Vector2};

use self::boxfilter::BoxFilter;

pub mod boxfilter;
pub enum Filter{
    BoxFilter(BoxFilter),
}
impl Filter{
    pub fn evaluate(&self, p: Point2)->f64{
        match *self {
            Self::BoxFilter(v)=>v.evaluate(p)
        }
    }
    pub fn get_radius(&self) -> Vector2 {
        match *self {
            Self::BoxFilter(v)=>v.get_radius()
        }
    }
}