pub type Point3 = cgmath::Point3<f64>;
pub type Vector3 = cgmath::Vector3<f64>;
pub type Point2 = cgmath::Point2<f64>;
pub type Vector2 = cgmath::Vector2<f64>;
pub type Mat4 = cgmath::Matrix4<f64>;
pub fn p_to_v(p: Point3) -> Vector3 {
    Vector3::new(p.x, p.y, p.z)
}
pub static ONES: Mat4 = Mat4::new(
      1.0, 0.0, 0.0, 0.0,
      0.0, 1.0, 0.0, 0.0,
      0.0, 0.0, 1.0, 0.0, 
      0.0, 0.0, 0.0, 1.0,
);
// use std::ops::{Add, Div, Index, Mul, Neg, Sub};

// use cgmath::{Deg, InnerSpace, Matrix, Rad, SquareMatrix, Transform};
// #[derive(Clone, Copy, Debug)]
// pub struct Point3(cgmath::Point3<f64>);
// #[derive(Clone, Copy, Debug)]
// pub struct Vector3(cgmath::Vector3<f64>);
// #[derive(Clone, Copy, Debug)]
// pub struct Normal3(cgmath::Vector3<f64>);
// #[derive(Clone, Copy, Debug)]
// pub struct Mat4(cgmath::Matrix4<f64>);
// #[derive(Clone, Copy, Debug)]
// pub struct Point2(cgmath::Point2<f64>);

// impl Index<usize> for Point2 {
//     type Output = f64;
//     fn index(&self, index: usize) -> &Self::Output {
//         match index {
//             0 => &self.0.x,
//             1 => &self.0.y,
//             _ => panic!(),
//         }
//     }
// }
// impl Index<usize> for Point3 {
//     type Output = f64;
//     fn index(&self, index: usize) -> &Self::Output {
//         match index {
//             0 => &self.0.x,
//             1 => &self.0.y,
//             2 => &self.0.z,
//             _ => panic!(),
//         }
//     }
// }
// impl Index<usize> for Vector3 {
//     type Output = f64;
//     fn index(&self, index: usize) -> &Self::Output {
//         match index {
//             0 => &self.0.x,
//             1 => &self.0.y,
//             2 => &self.0.z,
//             _ => panic!(),
//         }
//     }
// }
// impl Div<Vector3> for f64 {
//     fn div(self, rhs: Vector3) -> Self::Output {
//         Vector3(self / rhs.0)
//     }

//     type Output = Vector3;
// }
// impl Mul<f64> for Point2 {
//     type Output = Point2;
//     fn mul(self, rhs: f64) -> Self::Output {
//         Self(self.0 * rhs)
//     }
// }
// impl Point2 {
//     pub fn new(x: f64, y: f64) -> Self {
//         Self(cgmath::Point2 { x: (x), y: (y) })
//     }
// }
// impl Mul<f64> for Point3 {
//     type Output = Point3;
//     fn mul(self, rhs: f64) -> Self::Output {
//         Self(self.0 * rhs)
//     }
// }
// impl Add<Vector3> for Point3 {
//     type Output = Vector3;
//     fn add(self, rhs: Vector3) -> Self::Output {
//         Point3(self.0 + rhs.0)
//     }
// }
// impl Add<Point3> for Vector3 {
//     type Output = Point3;
//     fn add(self, rhs: Point3) -> Self::Output {
//         Point3(rhs.0 + self.0)
//     }
// }
// impl Add<Point3> for Point3 {
//     type Output = Point3;
//     fn add(self, rhs: Point3) -> Self::Output {
//         let x = self.0.x + rhs.0.x;
//         let y = self.0.y + rhs.0.y;
//         let z = self.0.z + rhs.0.z;
//         Self(cgmath::Point3::new(x, y, z))
//     }
// }
// impl Neg for Point3 {
//     type Output = Point3;
//     fn neg(self) -> Self::Output {
//         let x=-self.0.x;
//         let y=-self.0.y;
//         let z=-self.0.z;
//         Self(cgmath::Point3 { x: x, y: y, z: z })

//     }
// }
// impl Sub<Point3> for Point3 {
//     type Output = Vector3;
//     fn sub(self, rhs: Point3) -> Self::Output {
//         Vector3(rhs.0 - self.0)
//     }
// }
// impl Point3 {
//     pub const ZERO: Point3 = Point3(cgmath::Point3 {
//         x: (0.0),
//         y: 0.0,
//         z: 0.0,
//     });
//     pub const X: Point3 = Point3(cgmath::Point3 {
//         x: (1.0),
//         y: 0.0,
//         z: 0.0,
//     });pub const Y: Point3 = Point3(cgmath::Point3 {
//         x: (0.0),
//         y: 1.0,
//         z: 0.0,
//     });pub const Z: Point3 = Point3(cgmath::Point3 {
//         x: (0.0),
//         y: 0.0,
//         z: 1.0,
//     });
//     pub fn new(x: f64, y: f64, z: f64) -> Self {
//         Self(cgmath::Point3 {
//             x: (x),
//             y: (y),
//             z: (z),
//         })
//     }
//     pub fn normalize(&self)->Vector3{
//         let sum=(self.0.x*self.0.x+self.0.y*self.0.y+self.0.z*self.0.z).sqrt();
//         Vector3(cgmath::Vector3 { x: (self.0.x/sum), y: self.0.y/sum, z: self.0.z/sum })
//     }
// }

// impl Add<Self> for Vector3 {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self::Output {
//         Self(self.0 + rhs.0)
//     }
// }
// impl Sub<Self> for Vector3 {
//     type Output = Self;
//     fn sub(self, rhs: Self) -> Self::Output {
//         Self(self.0 - rhs.0)
//     }
// }
// impl Sub<Point3> for Vector3 {
//     type Output =  Vector3;
//     fn sub(self, rhs:Point3) -> Self::Output {
//         let x=self.0.x-rhs.0.x;
//         let y=self.0.y-rhs.0.y;
//         let z=self.0.z-rhs.0.z;

//         Vector3(cgmath::Vector3 { x: x, y: y, z: z })
//     }
// }
// impl Mul<f64> for Vector3 {
//     type Output = Self;
//     fn mul(self, rhs: f64) -> Self::Output {
//         Self(self.0 * rhs)
//     }
// }
// impl Div<f64> for Vector3 {
//     type Output = Self;
//     fn div(self, rhs: f64) -> Self::Output {
//         Self(self.0 / rhs)
//     }
// }

// impl Vector3 {
//     pub const ZERO: Vector3 = Vector3(cgmath::Vector3 {
//         x: (0.0),
//         y: (0.),
//         z: (0.0),
//     });
//     pub const ONE: Vector3 = Vector3(cgmath::Vector3 {
//         x: (1.0),
//         y: (1.),
//         z: (1.0),
//     });
//     pub const X: Vector3 = Vector3(cgmath::Vector3 {
//         x: (1.0),
//         y: (0.),
//         z: (0.0),
//     });
//     pub const Y: Vector3 = Vector3(cgmath::Vector3 {
//         x: (1.0),
//         y: (1.),
//         z: (0.0),
//     });
//     pub const Z: Vector3 = Vector3(cgmath::Vector3 {
//         x: (1.0),
//         y: (0.),
//         z: (1.0),
//     });

//     pub fn cross(&self, other: &Self) -> Self {
//         Self(self.0.cross(other.0))
//     }
//     pub fn dot(&self, other: &Self) -> f64 {
//         self.0.dot(other.0)
//     }
//     pub fn normalize(&self) -> Self {
//         Self(self.0.normalize())
//     }
//     pub fn new(x: f64, y: f64, z: f64) -> Self {
//         Self(cgmath::Vector3 {
//             x: (x),
//             y: (y),
//             z: (z),
//         })
//     }
// }
// impl Normal3 {
//     pub fn face_forward(&self, vec: &Vector3) -> Self {
//         if self.0.dot(vec.0) > 0.0 {
//             Self(self.0)
//         } else {
//             Self(-self.0)
//         }
//     }
//     pub fn dot_v(&self, vec: &Vector3) -> f64 {
//         self.0.dot(vec.0)
//     }
//     pub fn dot_n(&self, normal: &Normal3) -> f64 {
//         self.0.dot(normal.0)
//     }
//     pub fn normalize(&self)->Self{
//         Self(self.0.normalize())
//     }
// }

// impl Add<Self> for Normal3 {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self::Output {
//         Self(self.0 + rhs.0)
//     }
// }
// impl Sub<Self> for Normal3 {
//     type Output = Self;
//     fn sub(self, rhs: Self) -> Self::Output {
//         Self(self.0 - rhs.0)
//     }
// }
// impl Mul<f64> for Normal3 {
//     type Output = Self;
//     fn mul(self, rhs: f64) -> Self::Output {
//         Self(self.0 * rhs)
//     }
// }
// impl Div<f64> for Normal3 {
//     type Output = Self;
//     fn div(self, rhs: f64) -> Self::Output {
//         Self(self.0 / rhs)
//     }
// }
// impl Normal3 {
//     pub fn new(x: f64, y: f64, z: f64) -> Self {
//         Self(cgmath::Vector3 { x: x, y: y, z: z })
//     }
// }

// impl Add<Self> for Mat4 {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self::Output {
//         Self(self.0 + rhs.0)
//     }
// }
// impl Sub<Self> for Mat4 {
//     type Output = Self;
//     fn sub(self, rhs: Self) -> Self::Output {
//         Self(self.0 - rhs.0)
//     }
// }
// impl Mul<Self> for Mat4 {
//     type Output = Self;
//     fn mul(self, rhs: Self) -> Self::Output {
//         Self(self.0 * rhs.0)
//     }
// }
// impl Mat4 {
//     pub const IDENTITY: Mat4 = Self(cgmath::Matrix4::<f64>::identity());
//     pub fn look_at(eye: Point3, center: Point3, up: Vector3) -> Self {
//         Self(cgmath::Matrix4::look_at_lh(eye.0, center.0, up.0))
//     }
//     pub fn inverse(&self) -> Self {
//         Self(self.0.inverse_transform().expect("你的原矩阵没有逆"))
//     }
//     pub fn transpose(&self) -> Self {
//         Self(self.0.transpose())
//     }
//     pub fn is_identity(&self) -> bool {
//         self.0.is_identity()
//     }
//     pub fn from_scale(x: f64, y: f64, z: f64) -> Self {
//         Self(cgmath::Matrix4::from_nonuniform_scale(x, y, z))
//     }
//     pub fn from_translation(v: Vector3) -> Self {
//         Self(cgmath::Matrix4::from_translation(v.0))
//     }
//     pub fn from_rotation_x(theta: f64) -> Self {
//         Self(cgmath::Matrix4::from_angle_x(Rad::from(Deg(theta))))
//     }
//     pub fn from_rotation_y(theta: f64) -> Self {
//         Self(cgmath::Matrix4::from_angle_y(Rad::from(Deg(theta))))
//     }
//     pub fn from_rotation_z(theta: f64) -> Self {
//         Self(cgmath::Matrix4::from_angle_z(Rad::from(Deg(theta))))
//     }
//     pub fn transform_vector(&self, vec: &Vector3) -> Vector3 {
//         Vector3(self.0.transform_vector(vec.0))
//     }
//     pub fn transform_point(&self, point: &Point3) -> Point3 {
//         Point3(self.0.transform_point(point.0))
//     }
//     pub fn transform_normal(&self, normal: &Normal3) -> Normal3 {
//         Normal3(self.0.transform_vector(normal.0))
//     }
//     pub fn orthographic()->Self{
//         todo!()
//     }
//     pub fn perspective(fov:f64,aspect:f64,near:f64,far:f64)->Self{

//         todo!()
//     }
// }
