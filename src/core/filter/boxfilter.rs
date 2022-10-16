use crate::extends::{Vector2, Point2};


#[derive(Debug,  Copy, Clone)]
pub struct BoxFilter {
    pub radius: Vector2,
    pub inv_radius: Vector2,
}
impl Default for BoxFilter{
    fn default() -> Self {
        Self { radius: Vector2::new(0.0,0.0), inv_radius:Vector2::new(0.0,0.0) }
    }
}
impl BoxFilter {
    pub fn new(xwidth:f64,ywidth:f64) -> BoxFilter {
        Self {
            radius: Vector2 { x: xwidth, y: ywidth },
            inv_radius: Vector2 {
                x: 1.0 / xwidth,
                y: 1.0 / ywidth,
            },
        }
    }
    pub fn evaluate(&self, _p: Point2) -> f64 {
        1.0
    }
    pub fn get_radius(&self) -> Vector2 {
        Vector2 {
            x: self.radius.x,
            y: self.radius.y,
        }
    }
}
