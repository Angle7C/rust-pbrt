use std::ops::{Add, Mul, Bound};

use crate::{extends::{Mat4, Vec3, Point3, Normal}, core::{ray::Ray, aabb::{Bounds3}}};
#[derive(Clone, Copy)]
pub struct  Transform{
    pub trans:Mat4,
    pub inv_trans:Mat4,
}
impl Transform{
    pub const IDENTITY:Self =Self{trans:Mat4::IDENTITY,inv_trans:Mat4::IDENTITY};
    pub fn new(trans:Mat4)->Self{
        Self { trans: (trans), inv_trans: (trans.inverse()) }
    }
    pub fn inverse(&self)->Self{
        Self { trans: (self.inv_trans), inv_trans: (self.trans) }
    }
    pub fn transpose(&self)->Self{
        Self { trans: (self.trans.transpose()), inv_trans: (self.inv_trans.transpose()) }
    }
    pub fn scaling(&self,x :f32,y:f32,z:f32)->Self{
        let mat = Mat4::from_scale(Vec3::new(x,y,z));
        let tran=self.trans*mat;
        Self{
            trans: tran,
            inv_trans:tran.inverse(),
        }
    }
    pub fn translations(&self,x:f32,y:f32,z:f32)->Self{
        let mat = Mat4::from_translation(Vec3::new(x,y,z));
        let tran=self.trans*mat;
        Self{
            trans: tran,
            inv_trans:tran.inverse(),
        }
    }
    pub fn rotation_x(&self,angle:f32)->Self{
        let mat=Mat4::from_rotation_x(angle);
        let tran=self.trans*mat;
        Self{
            trans: tran,
            inv_trans:tran.inverse(),
        }
    }
    pub fn rotation_y(&self,angle:f32)->Self{
        let mat=Mat4::from_rotation_y(angle);
        let tran=self.trans*mat;
        Self{
            trans: tran,
            inv_trans:tran.inverse(),
        }
    }
    pub fn rotation_z(&self,angle:f32)->Self{
        let mat=Mat4::from_rotation_z(angle);
        let tran=self.trans*mat;
        Self{
            trans: tran,
            inv_trans:tran.inverse(),
        }
    }
    pub fn rotation(&self,angle:f32,axis:Vec3)->Self{
        let mat=Mat4::from_axis_angle(axis,angle);
        let tran=self.trans*mat;
        Self{
            trans: tran,
            inv_trans:tran.inverse(),
        }
    }
    pub fn look_at_lh(eye: Vec3, center: Vec3, up: Vec3)->Self{
        let camera=Mat4::look_at_lh(eye, center, up);
        Self { trans: (camera), inv_trans: (camera.inverse()) }
    }
    pub fn applying_point(&self,p:Point3)->Point3{
        self.trans.transform_point3(p)
    }
    pub fn applying_vector(&self,p:Vec3)->Vec3{
        self.trans.transform_vector3(p)
    }
    pub fn applying_normal(&self,normal:Normal)->Normal{
        let mat=self.inverse().transpose();
        mat.trans.transform_vector3(normal)
    }
    pub fn applying_ray(&self,ray:&Ray)->Ray{
        let o=self.applying_point(ray.o);
        let d=self.applying_vector(ray.d);
        Ray::init_o_dir(o, d)
    }
    pub fn applying_box_3(&self,bounds:&mut Bounds3)->Bounds3{
        let mut init=Bounds3::init_point(self.inv_trans.transform_point3( bounds.rang_point(0)));
        for i in 1..8{
            init=init.union_point(self.inv_trans.transform_point3(bounds.rang_point(i)));
        }
        init
    }

    pub fn applying_point_inv(&self,p:Point3)->Point3{
        self.inv_trans.transform_point3(p)
    }
    pub fn applying_vector_inv(&self,p:Vec3)->Vec3{
        self.inv_trans.transform_vector3(p)
    }
    pub fn applying_normal_inv(&self,normal:Normal)->Normal{
        let mat=self.inverse().transpose();
        mat.inv_trans.transform_vector3(normal)
    }
    pub fn applying_ray_inv(&self,ray:&Ray)->Ray{
        let o=self.applying_point_inv(ray.o);
        let d=self.applying_vector_inv(ray.d);
        Ray::init_o_dir(o, d)
    }
    pub fn applying_box_inv_3(&self,bounds:&mut Bounds3)->Bounds3{
        let mut init=Bounds3::init_point(self.inv_trans.transform_point3( bounds.rang_point(0)));
        for i in 1..8{
            init=init.union_point(self.inv_trans.transform_point3(bounds.rang_point(i)));
        }
        init
    }

}
impl Mul<Transform> for Transform{
    type Output = Transform;
    fn mul(self, rhs: Transform) -> Self::Output {
        let trans=self.trans*rhs.trans;
        let inv_trans=trans.inverse();
        Self{
            trans,
            inv_trans
        }
        
    }
}
