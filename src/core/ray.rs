
use cgmath::{EuclideanSpace, Zero};

use crate::extends::{Point3, Vector3};

use super::medium::Medium;

#[derive(Clone,Debug)]
pub struct Ray {
    /// origin
    pub o: Point3,
    /// direction
    pub d: Vector3,
    /// limits the ray to a segment along its infinite extent
    pub t_max: f64,
    /// used for animations
    pub time: f64,
    pub medium: Option<Medium>,
    /// in C++: 'class RayDifferential : public Ray'
    pub differential: Option<RayDifferential>,
}
impl Default for Ray {
    fn default() -> Self {
        Self {
            o: Point3::origin(),
            d: Vector3::zero(),
            t_max: f64::INFINITY,
            time: 0.0,
            medium:None,
            differential: None,
        }
    }
}
impl Ray {
    // Point3f operator()(Float t) const { return o + d * t; }
    pub fn at(&self, t: f64) -> Point3 {
        self.o + self.d * t
    }
    pub fn new(o:Point3,v:Vector3)->Self{
        Self {
            o: o,
            d: v,
            t_max: f64::INFINITY,
            time: 0.0,
            medium:None,
            differential: None,
        }
    }
    // from class RayDifferential
    pub fn scale_differentials(&mut self, s: f64) {
        if let Some(d) = self.differential.iter_mut().next() {
            d.rx_origin = self.o + (d.rx_origin - self.o) * s;
            d.ry_origin = self.o + (d.ry_origin - self.o) * s;
            d.rx_direction = self.d + (d.rx_direction - self.d) * s;
            d.ry_direction = self.d + (d.ry_direction - self.d) * s;
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RayDifferential {
    pub rx_origin: Point3,
    pub ry_origin: Point3,
    pub rx_direction: Vector3,
    pub ry_direction: Vector3,
}
