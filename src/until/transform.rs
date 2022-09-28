use std::ops::Mul;

use cgmath::{Matrix, Rad, SquareMatrix, Transform, Deg};

use crate::{
    core::{aabb::Bounds3, ray::Ray},
    extends::{Mat4, Point3, Vector3},
};
#[derive(Clone, Debug, Copy)]
pub struct Transforms {
    pub trans: Mat4,
    pub inv_trans: Mat4,
}
impl Default for Transforms {
    fn default() -> Self {
        Self::IDENTITY
    }
}
impl Transforms {
    pub const IDENTITY: Self = Self {
        trans: cgmath::Matrix4 {
            x: cgmath::Vector4 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            y:  cgmath::Vector4 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
                w: 0.0,
            },
            z:  cgmath::Vector4 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
                w: 0.0,
            },
            w:  cgmath::Vector4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
        },
        inv_trans: cgmath::Matrix4 {
            x: cgmath::Vector4 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            y:  cgmath::Vector4 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
                w: 0.0,
            },
            z:  cgmath::Vector4 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
                w: 0.0,
            },
            w:  cgmath::Vector4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
        },
    };
    pub fn new(trans: Mat4) -> Self {
        Self {
            trans: (trans),
            inv_trans: (trans.invert().unwrap()),
        }
    }
    pub fn inverse(&self) -> Self {
        Self {
            trans: (self.inv_trans),
            inv_trans: (self.trans),
        }
    }
    pub fn transpose(&self) -> Self {
        Self {
            trans: (self.trans.transpose()),
            inv_trans: (self.inv_trans.transpose()),
        }
    }
    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        let mat = Mat4::from_nonuniform_scale(x, y, z);
        Self {
            trans: mat,
            inv_trans: mat.invert().unwrap(),
        }
    }
    pub fn translations(x: f64, y: f64, z: f64) -> Self {
        let tran = Mat4::from_translation(Vector3::new(x, y, z));
        Self {
            trans: tran,
            inv_trans: tran.invert().unwrap(),
        }
    }
    pub fn rotation_x(&self, angle: f64) -> Self {
        let tran = Mat4::from_angle_x(Rad(angle));

        Self {
            trans: tran,
            inv_trans: tran.invert().unwrap(),
        }
    }
    pub fn rotation_y(&self, angle: f64) -> Self {
        let tran = Mat4::from_angle_y(Rad(angle));
        Self {
            trans: tran,
            inv_trans: tran.invert().unwrap(),
        }
    }
    pub fn rotation_z(&self, angle: f64) -> Self {
        let tran = Mat4::from_angle_z(Rad(angle));
        Self {
            trans: tran,
            inv_trans: tran.invert().unwrap(),
        }
    }
    pub fn look_at_lh(eye: Point3, center: Point3, up: Vector3) -> Self {
        let camera = Mat4::look_at_lh(eye, center, up);
        Self {
            trans: (camera),
            inv_trans: (camera.invert().unwrap()),
        }
    }
    pub fn applying_point(&self, p: Point3) -> Point3 {
        self.trans.transform_point(p)
    }
    pub fn applying_vector(&self, p: Vector3) -> Vector3 {
        self.trans.transform_vector(p)
    }
    pub fn applying_normal(&self, normal: Vector3) -> Vector3 {
        let mat = self.inverse().transpose();
        mat.trans.transform_vector(normal)
    }
    pub fn applying_ray(&self, ray: &Ray) -> Ray {
        let o = self.applying_point(ray.o);
        let d = self.applying_vector(ray.d);
        Ray::new(o, d)
    }
    pub fn applying_box_3(&self, bounds: &Bounds3) -> Bounds3 {
        let mut init = Bounds3::init_point(self.inv_trans.transform_point(bounds.rang_point(0)));
        for i in 1..8 {
            init = init.union_point(self.inv_trans.transform_point(bounds.rang_point(i)));
        }
        init
    }

    pub fn applying_point_inv(&self, p: Point3) -> Point3 {
        self.inv_trans.transform_point(p)
    }
    pub fn applying_vector_inv(&self, p: Vector3) -> Vector3 {
        self.inv_trans.transform_vector(p)
    }
    pub fn applying_normal_inv(&self, normal: Vector3) -> Vector3 {
        let mat = self.inverse().transpose();
        mat.inv_trans.transform_vector(normal)
    }
    pub fn applying_ray_inv(&self, ray: &Ray) -> Ray {
        let o = self.applying_point_inv(ray.o);
        let d = self.applying_vector_inv(ray.d);
        Ray::new(o, d)
    }
    pub fn applying_box_inv_3(&self, bounds: &mut Bounds3) -> Bounds3 {
        let mut init = Bounds3::init_point(self.inv_trans.transform_point(bounds.rang_point(0)));
        for i in 1..8 {
            init = init.union_point(self.inv_trans.transform_point(bounds.rang_point(i)));
        }
        init
    }
    pub fn perspective(fov: f64, n: f64, f: f64) -> Transforms {
       let mat4=Mat4::new(
        1.0,0.0,0.0,0.0,
        0.0,1.0,0.0,0.0,
        0.0,0.0,f/(f-n),-f*n/(f-n),
        0.0,0.0,1.0,1.0,
       );
       let inv_tan=1.0/Rad::from(Deg(fov/2.0)).0.tan();
       let per=Mat4::from_nonuniform_scale(inv_tan, inv_tan, 1.0)*mat4;
       Transforms::new(per)

    }
}
impl Mul<Transforms> for Transforms {
    type Output = Transforms;
    fn mul(self, rhs: Transforms) -> Self::Output {
        let trans = self.trans * rhs.trans;
        let inv_trans = trans.invert().unwrap();
        Self { trans, inv_trans }
    }
}
