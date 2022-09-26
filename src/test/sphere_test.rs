#[cfg(test)]
mod test {
    use crate::{
        core::{
            ray::Ray,
            shape::{sphere::Sphere, BaseShapeAble},
        },
        extends::{Mat4, Point3, Vec3},
    };
    use std::f32::consts::PI;

    #[test]
    fn test_sphere() {
        let sphere = Sphere::new(Mat4::IDENTITY, false, 0.5, -1.0, 1.0, 2.0 * PI);
        const WIDTH: i32 = 100;
        const HEIGHT: i32 = 100;
        let mut ray = Ray::new(Point3::ZERO, Vec3::Z, 200.0, 0.0, None);
        let mut image = image::RgbImage::new(WIDTH as u32, HEIGHT as u32);
        let mut u: f32;
        let mut v: f32;
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                u = (i - WIDTH / 2) as f32 / WIDTH as f32;
                v = (j - HEIGHT / 2) as f32 / HEIGHT as f32;
                ray.o = Point3::new(u, v, -2.0);
                match sphere.intersect(&ray) {
                    None => continue,
                    Some(ref v) => {
                        let color = (v.normal + Vec3::ONE) * 0.5 * 255.0;
                        *image.get_pixel_mut(i as u32, j as u32) =
                            image::Rgb([color.x as u8, color.y as u8, color.z as u8])
                    }
                }
            }
        }
        image.save("sphere.png").unwrap();
    }
}
