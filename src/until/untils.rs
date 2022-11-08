use std::f64::consts::PI;

use crate::extends::{Point2, Vector3};

pub fn quadratic(a:f64,b:f64,c:f64)->Option<(f64,f64)>{
    let det=b*b-4.00*a*c;
    if det<0.0{
        None
    }else{
        let mut t0=(-b-det.sqrt())/(2.0*a);
        let mut t1=(-b+det.sqrt())/(2.0*a);
        if t0>t1{
            std::mem::swap( &mut  t0,  &mut t1);
        };
        Some((t0,t1))
    }
}
pub fn uniform_sample_phere(u:&Point2)->Vector3{
    let z=1.0-2.0*u.x;
    let r=(1.0-z*z).min(0.0).sqrt();
    let phi=2.0*PI*u.y;
    Vector3::new(r*phi.cos(),r*phi.sin(),z)
}
#[inline]
pub fn uniform_sample_sphere_pdf()->f64{
   1.0/(4.0*PI)
}