#[cfg(test)]
mod test {
    use cgmath::{ EuclideanSpace};

    use crate::{
        core::{
            light::{pointlight::PointLight, Light},
            scene::Scene,
            shape::{sphere::Sphere, Shape},
            spectrum::RGBSpectrum,
        },
        extends::{self, Mat4, Vector3, Point3},
    };

    #[test]
    pub fn test_sence() {
        let mut sence = Scene::new();
        sence.add_shape(Shape::Sphere(Sphere::new(
            Mat4::from_translation(Vector3::new(-0.0, -0.0, -0.0)),
            false,
            100.0,
            -100.0,
            100.0,
            360.0,
        )));
        sence.add_shape(Shape::Sphere(Sphere::new(
            Mat4::from_translation(Vector3::new(1.0,0.0,0.0)),
            false,
            1.0,
            -1.0,
            1.0,
            360.0,
        )));
        sence.add_light(Light::PointLight(Box::new(PointLight::new(
            Mat4::from_translation(Vector3::new(0.0, 60.0, 0.0)),
            None,
            RGBSpectrum::new(255.0, 255.0, 255.0),
        ))));
        sence.set_camera(Point3::new(0.0,0.0,10.0),Point3::origin(),"scence_test2.png");
        sence.render()
    }
}
