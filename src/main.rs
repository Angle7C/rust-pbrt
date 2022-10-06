use cgmath::{Transform, Point3, EuclideanSpace};
use extends::{Mat4, Vector3};

pub mod core;
pub mod extends;
pub mod until;
pub mod test;
fn main() {
    // let mut t=vec![cgmath::Vector3::<f32>::new(0.0,0.0,0.0)];\
    let mat1=Mat4::from_translation(Vector3::new(10.0,10.0,10.0));
    let mat2=Mat4::from_scale(10.0);
    let t = mat1.transform_point(Point3::origin());
    println!("{:?}",t);
    let t = (mat1*mat2).transform_point(t);
    println!("{:?}",t);

}
