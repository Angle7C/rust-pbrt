use crate::{
    extends::{Mat4, Point2, Point3, Vector3},
    until::transform::Transforms,
};

use super::{
    aabb::Bounds3,
    camera::{perspecttivecamera::PerspectiveCamera, Camera},
    film::Film,
    interaction::SurfaceInteraction,
    light::Light,
    primitives::{bvh::BVH, Primitive},
    shape::Shape,
    spectrum::RGBSpectrum, ray::Ray,
};

pub struct Scene {
    pub shapes: Vec<Shape>,
    pub lights: Vec<Light>,
    pub primitives: Vec<Primitive>,
    pub camera: Camera,
}
impl<'a> Scene {
    pub fn new() -> Self {
        Self {
            shapes: vec![],
            lights: vec![],
            primitives: vec![],
            camera: Camera::Nil,
        }
    }
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light)
    }
    pub fn add_shape(&mut self, shape: Shape) {
        let p = Primitive::new(self.shapes.len(), None, None, None);
        self.shapes.push(shape);
        self.primitives.push(p);
    }
    pub fn set_camera(&mut self, pos: Point3, dir: Point3, filename: &'static str) {
        self.camera = Camera::Perspective(Box::new(PerspectiveCamera::new(
            Transforms::new(Mat4::look_at_lh(pos, dir, Vector3::unit_y())),
            Bounds3::new(Point3::new(-1.0, -1.0, 0.0), Point3::new(1.0, 1.0, 0.0)),
            0.0,
            1.0,
            0.0,
            1.0,
            90.0,
            Film::new(Point2::new(400.0, 400.0), filename),
            None,
            0.0,
        )));
    }
    pub fn render(&mut self) {
        let bvh = BVH::build(&mut self.primitives, &self.shapes);
        while let Some(ref sample) = self.camera.next_camsample() {
            let mut ray = self.camera.generate_ray(sample);
            if let Some(t) = bvh.hit_BVH(&ray, &self.primitives) {
                let mut isect = SurfaceInteraction::init();
                let mut state = false;
                for i in t {
                    if self.shapes[self.primitives[i].shape_index].intersect(&mut ray,&mut isect) {
                        state = true;
                    }
                }
                if state {
                    let color = (isect.normal + Vector3::new(1.0, 1.0, 1.0)) * 0.5;
                    self.camera
                        .set_pixel(sample, RGBSpectrum::new(color.x, color.y, color.z));
                }
            }
        }
        self.camera.output_image();
    }
    //求解交点，
    fn slove_point(&self,ray:&mut Ray)->Option<SurfaceInteraction<'a>>{
        unimplemented!();
    }
    //对光源采样
    fn light_sample(&self,surfaceInteraction:Option<SurfaceInteraction<'a>>){

    }
    //
}
#[cfg(test)]
mod test {
    use cgmath::{EuclideanSpace, Point3};

    use crate::{
        core::{
            light::Light,
            shape::{sphere::Sphere, Shape},
        },
        extends::{Mat4, ONES},
    };

    use super::Scene;

    #[test]
    fn test_scence() {
        let mut scene = Scene::new();
        scene.add_light(Light::Nil);
        scene.add_shape(Shape::Sphere(Sphere::new(
            ONES, false, 2.0, 1.5, -1.5, 360.0,
        )));
        scene.set_camera(
            Point3::new(5.0, 5.0, 5.0),
            Point3::origin(),
            "scence_test.png",
        );
        scene.render();
    }
}
