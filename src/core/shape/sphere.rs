use super::*;
use crate::{
    core::{ray::RayAble, sample::Sample},
    extends::*,
    until::untils::quadratic,
};

pub struct Sphere {
    shape: BaseShape,
    pub radius: f32,
    pub theta_min: f32,
    pub theta_max: f32,
    pub phi_max: f32,
    pub z_min: f32,
    pub z_max: f32,
}
impl Sphere {
    pub fn new(
        object_to_world: Mat4,
        reverse_orientation: bool,
        radius: f32,
        z_min: f32,
        z_max: f32,
        phi_max: f32,
    ) -> Self {
        let base = BaseShape::new(object_to_world, reverse_orientation);
        let z_min = z_min.min(z_max).clamp(-radius, radius);
        let z_max = z_min.max(z_max).clamp(-radius, radius);
        let theta_min = ((z_min.min(z_max) / radius).clamp(-1.0, 1.0)).acos();
        let theta_max = ((z_min.max(z_max) / radius).clamp(-1.0, 1.0)).acos();
        let phi_max = phi_max.clamp(0.0, 360.0);
        Self {
            shape: (base),
            radius: (radius),
            theta_min: (theta_min),
            theta_max: (theta_max),
            phi_max: (phi_max),
            z_min: (z_min),
            z_max: (z_max),
        }
    }
}
 impl BaseShapeAble for Sphere {
    fn area(&self) -> f32 {
        self.radius * self.phi_max * (self.z_max - self.z_min)
    }
    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        let ray=self.obj_to_world().applying_ray_inv(ray);
        let a = ray.d.x * ray.d.x + ray.d.y * ray.d.y + ray.d.z * ray.d.z;
        let b = 2.0 * (ray.d.x * ray.o.x + ray.d.y * ray.o.y + ray.d.z * ray.o.z);
        let c =
            ray.o.x * ray.o.x + ray.o.y * ray.o.y + ray.o.z * ray.o.z - self.radius * self.radius;
        //求解圆和射线的交点。
        if let Some((t0, t1)) = quadratic(a, b, c) {
            if t0 >= ray.t_max || t1 < 0.0 {
                return None;
            };
            Some(SurfaceInteraction::init(
                Point3::ZERO,
                0.0,
                Point3::ZERO,
                Point3::ZERO,
                None,
                UV::ZERO,
                Point3::ZERO,
                Point3::ZERO,
                Point3::ZERO,
                Point3::ZERO,
            ))
        } else {
            None
        }
    }
    fn intersect_p(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        self.intersect(ray)
    }
    fn obj_to_world(&self) -> Transform {
        self.shape.obj_to_world
    }
    fn object_world_bound(&self) -> Bounds3 {
        Bounds3 {
            min: Point3::new(-self.radius, -self.radius, self.z_min),
            max: Point3::new(self.radius, self.radius, self.z_max),
        }
    }
    fn object_bound(&self) -> Bounds3 {
        Bounds3::new(
            Point3::new(-self.radius, -self.radius, self.z_min),
            Point3::new(self.radius, self.radius, self.z_max),
        )
    }
    fn pdf(&self, interaction: &Interaction) -> f32 {
        1.0/self.area()
    }
    fn pdf_iter(&self, interaction: &Interaction, wi: &Vec3) -> f32 {
        1.0/self.area()
        
    }
    fn reverse_orientation(&self) -> bool {
        false
        
    }
    fn sample(&self, u: &Point2) -> (Interaction, f32) {
        let mut p_obj = Point3::ZERO + self.radius * Sample::sphere_sample_uniform(u);
        let mut it = Interaction::init();
        it.normal = self
            .obj_to_world()
            .applying_point(Vec3::new(p_obj.x, p_obj.y, p_obj.z))
            .normalize();
        if self.reverse_orientation() {
            it.normal = -it.normal;
        }
        p_obj = p_obj * (self.radius / p_obj.distance(Point3::ZERO));
        it.p = self.obj_to_world().applying_point(p_obj);
        let pdf = 1.0 / self.area();
        (it, pdf)
    }
    fn sample_inter(&self, interaction: &Interaction, u: &Point2) -> (Interaction, f32) {
        self.sample(u)
    }
    fn transform_swap_handedness(&self) -> bool {
        false
        
    }
}
