
pub  type Point3=glam::Vec3;
pub  type Point2=glam::Vec2;
pub  type Vec3=glam::Vec3;
pub  type Vec2=glam::Vec2;
pub  type Normal=glam::Vec3;
pub  type Affine3=glam::Affine3A;
pub  type UV=glam::UVec2;
pub  type Mat4=glam::Mat4;

#[inline]
pub fn lerp(t:f32,s:f32,e:f32)->f32{
    s*(1.0-t)+e*t
}