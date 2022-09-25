use crate::{
    core::ray::RayAble,
    extends::Mat4,
    until::{transform::Transform, untils::quadratic},
};

use super::*;

pub struct Cylinders {
    pub shape: BaseShape,
    pub radius: f32,
    pub z_min: f32,
    pub z_max: f32,
    pub phi_max: f32,
}
impl Cylinders {
    pub fn new(
        obj_to_world: Mat4,
        reverser_orientation: bool,
        radius: f32,
        z_min: f32,
        z_max: f32,
        phi_max: f32,
    ) -> Self {
        let base = BaseShape::new(obj_to_world, reverser_orientation);
        Self {
            shape: base,
            radius: radius,
            z_min: z_min,
            z_max: z_max,
            phi_max: (phi_max / 180.0 * PI).clamp(0.0, 2.0 * PI),
        }
    }
}
impl BaseShapeAble for Cylinders {
    fn obj_to_world(&self) -> Transform {
        self.shape.obj_to_world
    }
    fn reverse_orientation(&self) -> bool {
        self.shape.reverse_orientation
    }
    fn transform_swap_handedness(&self) -> bool {
        self.shape.transform_swap_handedness
    }
    fn object_bound(&self) -> Bounds3 {
        Bounds3::new(
            Vec3::new(-self.radius, -self.radius, self.z_min),
            Vec3::new(self.radius, self.radius, self.z_max),
        )
    }
    fn object_world_bound(&self) -> Bounds3 {
        let bounds = self.object_bound();
        self.obj_to_world().applying_box_3(&bounds)
    }
    fn intersect(&self, ray: &Ray) -> Option<Interaction> {
        let ray = self.obj_to_world().applying_ray_inv(&ray);
        let a = ray.d.x * ray.d.x + ray.d.y * ray.d.y;
        let b = 2.0 * (ray.d.x * ray.o.x + ray.d.y * ray.o.y);
        let c = ray.o.x.powf(2.0) + ray.o.y.powf(2.0) - self.radius.powf(2.0);
        if let Some((t0, t1)) = quadratic(a, b, c) {
            if t0 >= ray.t_max || t1 < 0.0 {
                return None;
            };
            let mut t = t0;
            if t < 0.0 {
                t = t1;
                if t > ray.t_max {
                    return None;
                }
            }
            let mut point = ray.at(t);
            let hit_r = f32::sqrt(point.x * point.x + point.y * point.y);
            point.x *= self.radius / hit_r;
            point.y *= self.radius / hit_r;
            let mut phi = f32::atan2(point.y, point.x);
            if phi < 0.0 {
                phi += 2.0 * PI;
            };
            if (point.z < self.z_min || point.z > self.z_max || phi > self.phi_max) {
                if t == t1 || t1 > ray.t_max {
                    return None;
                }
                t = t1;
                point = ray.at(t);
                let hit_r = f32::sqrt(point.x * point.x + point.y * point.y);
                point.x *= self.radius / hit_r;
                point.y *= self.radius / hit_r;
                phi = point.y.atan2(point.x);
                if phi < 0.0 {
                    phi += 2.0 * PI;
                }
                if (point.z < self.z_min) || (point.z > self.z_max) || phi > self.phi_max {
                    return None;
                }
            }
            let u = phi / self.phi_max;
            let v = (point.z - self.z_min) / (self.z_max - self.z_min);
            let dpdu = Vec3::new(-self.phi_max * point.y, self.phi_max * point.x, 0.0);
            let dpdv = Vec3::new(0.0, 0.0, self.z_max - self.z_min);
            // let d2pduu=-self.phi_max*self.phi_max*Vec3::new(point.x, point.y, 0.0);
            // let d2pduv=Vec3::ZERO;
            // let d2pdvv=Vec3::ZERO;
            let normal = dpdv.cross(dpdu);
            let point = self.obj_to_world().applying_point(point);
            let w = self.obj_to_world().applying_vector(ray.d);
            let normal = self.obj_to_world().applying_vector(normal);
            Some(Interaction::new(point, t, w, normal))
        } else {
            None
        }
    }
    fn intersect_p(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        None
    }
    fn area(&self) -> f32 {
        (self.z_max - self.z_min) * self.radius * self.phi_max
    }
    fn pdf(&self, interaction: &Interaction) -> f32 {
        0.0
    }
    fn pdf_iter(&self, interaction: &Interaction, wi: &Vec3) -> f32 {
        self.pdf(interaction)
    }
    fn sample(&self, u: &Point2) -> (Interaction, f32) {
        (Interaction::init(), 0.0)
    }
    fn sample_inter(&self, interaction: &Interaction, u: &Point2) -> (Interaction, f32) {
        self.sample(u)
    }
}
#[cfg(test)]
mod test {
    use crate::core::aabb::Bounds2;
    use crate::core::camera::perspecttivecamera::PerspectiveCamera;
    use crate::core::camera::CameraAble;
    use crate::core::film::Film;
    use crate::core::shape::BaseShapeAble;
    use crate::core::spectrum::RGBSpectrum;
    use crate::extends::*;

    use super::Cylinders;
    #[test]
    fn test_camera_perspective() {
        let mut film = Film::new(Vec2::new(200.0, 200.0), "cylinders.png");
        let mut camera = PerspectiveCamera::new(
            Mat4::look_at_lh(Vec3::Y*2.0, Vec3::ZERO, Vec3::X),
            Bounds2::new(Vec2::new(-1.0, -1.0), Vec2::new(1.0, 1.0)),
            0.0,
            1.0,
            -0.0,
            1.0,
            90.0,
            &film,
            None,
        );
        let cylinders = Cylinders::new(Mat4::IDENTITY, false, 0.5, 0.0, 1.0, 360.0);
        let color = &RGBSpectrum::new(255.0, 0.0, 0.0);
        while let Some(v) = camera.next(&film) {
            if let (Some(ref ray), _) = camera.generate_ray(&v) {
                let t = cylinders.intersect(ray);
                match t {
                    None => continue,
                    Some(ref interaction) => {
                        let color = (interaction.normal + Vec3::ONE) * 255.0 * 0.5;
                        // film.set_pixel(v.p_film.x as u32, v.p_film.y as u32, color)
                        film.set_pixel(
                            v.p_film.x as u32,
                            v.p_film.y as u32,
                            &RGBSpectrum::new(color.x, color.y, color.z),
                        )
                    }
                }
            }
        }
        film.output_image();
    }
}
