use std::f32::consts::PI;

use euler::{Mat4, Vec3};
pub struct Ray {
    pub o: Vec3,
    pub dir: Vec3,
}
impl Ray {
    pub fn new(o: Vec3, dir: Vec3) -> Self {
        Ray {
            o: (o),
            dir: (dir).normalize(),
        }
    }
    pub fn ray_color(t_min: f32, t_max: f32) -> Vec3 {
        todo!();
    }
}
pub struct Camera {
    pub corner: Vec3,
    pub width: Vec3,
    pub height: Vec3,
    pub view_mat: Mat4,
    pub eye: Vec3,
}

impl Camera {
    //相机矩阵确定
    pub fn new(eye: Vec3, lookat: Vec3, up: Vec3, fov: f32, aspect: f32) -> Self {
        let dir = (eye - lookat).normalize();
        let right = dir.cross(up).normalize();
        let up = right.cross(dir).normalize();
        let theta = degress_to_rad(fov) / 2.0;
        let half_hight = theta.tan();
        let half_width = aspect * half_hight;
        let corner = Vec3::new(-half_hight, -half_width, -1.0);
        let width_dir = Vec3::new(2.0 * half_width, 0.0, 0.0);
        let height_dir = Vec3::new(0.0, 2.0 * half_hight, 0.0);
        let view = Mat4::new(
            right.x, right.y, right.z, 0.0, up.x, up.y, up.z, 0.0, dir.x, dir.y, dir.z, 0.0, 0.0,
            0.0, 0.0, 1.0,
        );
        let mut t = Mat4::identity();
        t.m03 = -eye.x;
        t.m13 = -eye.y;
        t.m23 = -eye.z;
        let view = view * t;
        Self {
            corner: corner,
            width: width_dir,
            height: height_dir,
            view_mat: view,
            eye: eye,
        }
    }
    //生成光线
    pub fn get_ray(&self, u: f32, v: f32) -> bvh::ray::Ray {
        let o = bvh::Vector3::new(self.eye.x, self.eye.y, self.eye.z);
        let dir = self.corner + u * self.width + v * self.height - self.eye;
        let dir = bvh::Vector3::new(dir.x, dir.y, dir.z);

        bvh::ray::Ray::new(o, dir)
    }
}
//角度值转成弧度制。
fn degress_to_rad(degress: f32) -> f32 {
    degress * (PI / 180.0)
}
#[cfg(test)]
mod test {
    use bvh::{bvh::BVH, ray::Ray};
    use euler::Vec3;
    use image::Rgb;
    use progress_bar::pb::ProgressBar;
    use PMXUtil::{
        reader::ModelInfoStage,
        types::{Face, Vertex},
    };

    use crate::lib::{extends::Triangle, observer::Camera};
    
    const PATH: &str = "../Pei_er/Fan.pmx";
    const WIDTH: u32 = 300;
    const HEIGHT: u32 = 300;

    #[test]
    fn test_pbx() {
        let info_stage = ModelInfoStage::open(PATH).expect("没有这个文件");
        let (header, vertex_stage) = info_stage.read();
        let (vertex, face_stage) = vertex_stage.read();
        let (face, _) = face_stage.read();
        println!("三角形数量{}", face.len());

        let mut image = image::RgbImage::new(WIDTH, HEIGHT);
        let mut list = Vec::<Triangle>::with_capacity(face.len());
        for item in &face {
            let t = Triangle::new(item, &vertex);
            list.push(t)
        }
        let mut u = 0.0;
        let mut v = 0.0;
        let camera = Camera::new(
            Vec3::new(-100000.0,-10000.0,0.0),
            Vec3::zero(),
            // Vec3::new(000000.0,500.0,0.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            1.5,
        );
        let mut pro = ProgressBar::new(WIDTH as usize);
        let bvh = BVH::build(&mut list);
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                u = i as f32 / WIDTH as f32;
                v = j as f32 / WIDTH as f32;
                let ray = camera.get_ray(u, v);
                let rec = bvh.traverse(&ray, &list);
                let color = if rec.len() != 0 {
                    println!("{}",rec.len());
                    Rgb::<u8>([255, 0, 0])
                } else {
                    Rgb::<u8>([0, 0, 255])
                };
                let piex = image.get_pixel_mut(i, j);
                *piex = color;
            }
            pro.inc();
        }
        image.save("test_model.png").unwrap();
    }
 
}
