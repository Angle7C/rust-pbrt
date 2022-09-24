mod test {
    use std::f32::consts::PI;

    use crate::{
        core::{aabb::Bounds2, camera::{perspecttivecamera::PerspectiveCamera, CameraAble, orthographiccamera::OrthographicCamera}, film::Film, shape::{sphere::Sphere, BaseShapeAble}, spectrum::RGBSpectrum},
        extends::{Mat4, Vec3,Vec2},
    };

    #[test]
    fn test_camera_perspective() {
        let mut film = Film::new(Vec2::new(100.0, 100.0), "test_camear.png");
        let mut camera = PerspectiveCamera::new(
            Mat4::look_at_lh(Vec3::Z*1.0, Vec3::ZERO, Vec3::Y),
            Bounds2::new(Vec2::new(-2.0, -2.0), Vec2::new(2.0, 2.0)),
            0.0,
            1.0,
            0.0,
            1.0,
            90.0,
            &film,
            None,
        );
        let sphere = Sphere::new(
            Mat4::IDENTITY,
            false,
            0.5,
            -1.0,
            1.0,
            2.0 * PI,
        );
        let color = &RGBSpectrum::new(255.0, 0.0, 0.0);
        while let Some(v) = camera.next(&film) {
            if let (Some(ref ray), _) = camera.generate_ray(&v) {
                let t = sphere.intersect(ray);
                match t {
                    None => continue,
                    _ => film.set_pixel(v.p_film.x as u32, v.p_film.y as u32, color),
                }
            }
        }
        film.output_image();
    }
    fn test_camera_orthographic() {
        let mut film = Film::new(Vec2::new(100.0, 100.0), "test_camear.png");
        let mut camera = OrthographicCamera::new(
            Mat4::look_at_lh(Vec3::ONE*1.0, Vec3::ZERO, Vec3::Y),
            Bounds2::new(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0)),
            0.0,
            1.0,
            0.0,
            1.0,
            &film,
            None,
        );
        let sphere = Sphere::new(
            Mat4::IDENTITY,
            false,
            0.5,
            -1.0,
            1.0,
            2.0 * PI,
        );
        let color = &RGBSpectrum::new(255.0, 0.0, 0.0);
        while let Some(v) = camera.next(&film) {
            if let (Some(ref ray), _) = camera.generate_ray(&v) {
                let t = sphere.intersect(ray);
                match t {
                    None => continue,
                    _ => film.set_pixel(v.p_film.x as u32, v.p_film.y as u32, color),
                }
            }
        }
        film.output_image();
    }

}
