use super::*;
use crate::{core::ray::RayAble, extends::*};
pub struct Disk {
    pub shape: BaseShape,
    pub radius: f32,
    pub inner_radius: f32,
    pub height: f32,
    pub phi_max: f32,
}
impl Disk {
    pub fn new(
        obj_to_world: Mat4,
        reverser_orientation: bool,
        radius: f32,
        inner_radius: f32,
        height: f32,
        phi_max: f32,
    ) -> Self {
        let shape = BaseShape::new(obj_to_world, reverser_orientation);
        Self {
            shape: shape,
            radius: radius,
            inner_radius: inner_radius,
            height: height,
            phi_max: (phi_max / 180.0 * PI).clamp(0.0, 2.0 * PI),
        }
    }
}
impl BaseShapeAble for Disk {
    fn area(&self) -> f32 {
        PI * (self.radius * self.radius - self.inner_radius * self.inner_radius) * 2.0 * PI
            / self.phi_max
    }
    fn object_bound(&self) -> Bounds3 {
        Bounds3::new(
            Point3::new(-self.radius, -self.radius, self.height),
            Point3::new(self.radius, self.radius, self.height),
        )
    }
    fn object_world_bound(&self) -> Bounds3 {
        let bounds = self.object_bound();
        self.obj_to_world().applying_box_3(&bounds)
    }
    fn obj_to_world(&self) -> Transform {
        self.shape.obj_to_world
    }
    fn intersect(&self, ray: &Ray) -> Option<Interaction> {
        let ray = self.obj_to_world().applying_ray_inv(&ray);
        let t = (self.height - ray.o.z) / ray.d.z;
        if t <= 0.0 || t > ray.t_max {
            return None;
        };
        let p = ray.at(t);
        let dist2 = p.x * p.x + p.y * p.y;
        if dist2 > self.radius * self.radius || dist2 < self.inner_radius * self.inner_radius {
            return None;
        };
        let mut phi = p.y.atan2(p.x);
        if phi < 0.0 {
            phi += 2.0 * PI;
        }
        if phi > self.phi_max {
            return None;
        };
        let point = self.obj_to_world().applying_point(p);
        let w = self.obj_to_world().applying_vector(ray.d);
        let normal = self.obj_to_world().applying_vector(Vec3::Z);
        Some(Interaction::new(point, t, w, normal))
    }
    fn intersect_p(&self, _ray: &Ray) -> Option<SurfaceInteraction> {
        None
    }
    fn pdf(&self, _interaction: &Interaction) -> f32 {
        0.0
    }
    fn pdf_iter(&self, interaction: &Interaction, _wi: &Vec3) -> f32 {
        self.pdf(interaction)
    }
    fn reverse_orientation(&self) -> bool {
        self.shape.reverse_orientation
    }
    fn sample(&self, _u: &Point2) -> (Interaction, f32) {
        todo!()
    }
    fn sample_inter(&self, _interaction: &Interaction, _u: &Point2) -> (Interaction, f32) {
        todo!()
    }
    fn transform_swap_handedness(&self) -> bool {
        self.shape.reverse_orientation
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

    use super::Disk;
    #[test]
    fn test_camera_perspective() {
        let mut film = Film::new(Vec2::new(2000.0, 2000.0), "Disk.png");
        let mut camera = PerspectiveCamera::new(
            Mat4::look_at_lh(Vec3::Z * 1.0, Vec3::ZERO, Vec3::Y),
            Bounds2::new(Vec2::new(-1.0, -1.0), Vec2::new(1.0, 1.0)),
            0.0,
            1.0,
            -0.0,
            1.0,
            90.0,
            &film,
            None,
        );
        let cylinders = Disk::new(Mat4::IDENTITY, false, 0.5, 0.49, 0.0, 360.0);
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
