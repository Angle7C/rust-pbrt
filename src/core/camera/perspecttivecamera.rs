// see perspective.h

use std::sync::Arc;

use cgmath::{EuclideanSpace, InnerSpace, SquareMatrix, Transform};

use crate::{
    core::{
        aabb::Bounds3,
        film::Film,
        interaction::Interaction,
        medium::Medium,
        ray::{Ray, RayDifferential},
        sample::{CameraSample, Sample},
        
        spectrum::RGBSpectrum,
    },
    extends::{p_to_v, Mat4, Point2, Point3, Vector3},
    until::transform::Transforms,
};

pub struct PerspectiveCamera {
    pub camera_to_world: Transforms,
    pub shutter_open: f64,
    pub shutter_close: f64,
    pub film: Arc<Film>,
    pub medium: Option<Medium>,
    pub raster_to_camera: Transforms,
    pub lens_radius: f64,
    pub focal_distance: f64,
    pub dx_camera: Vector3,
    pub dy_camera: Vector3,
    pub a: f64,
    clipping_start: f64, // ADDED
}

impl PerspectiveCamera {
    pub fn new(
        camera_to_world: Transforms,
        screen_window: Bounds3,
        shutter_open: f64,
        shutter_close: f64,
        lens_radius: f64,
        focal_distance: f64,
        fov: f64,
        film: Film,
        medium: Option<Medium>,
        clipping_start: f64,
    ) -> Self {
        // see perspective.cpp
        let camera_to_screen: Transforms = Transforms::perspective(fov, 1e-2, 1000.0, 1.0);
        //放大至图片像素
        let scale1 = Mat4::from_nonuniform_scale(
            film.full_resolution.x as f64,
            film.full_resolution.y as f64,
            1.0,
        );
        //放缩 到 [0,1][0,1]
        let scale2 = Mat4::from_nonuniform_scale(
            1.0 / (screen_window.max.x - screen_window.min.x),
            1.0 / (screen_window.min.y - screen_window.max.y),
            1.0,
        );
        //先将近平面的左下角移到原点
        let translate = Mat4::from_translation(Vector3::new(
            -screen_window.min.x,
            -screen_window.max.y,
            0.0,
        ));
        let screen_to_raster = scale1*scale2 * translate;
        let raster_to_screen = screen_to_raster.invert().unwrap();
        
        // let raster_to_screen = screen_to_raster.invert().unwrap();
        let raster_to_camera = camera_to_screen.inv_trans * raster_to_screen;

        let dx_camera: Vector3 = raster_to_camera.transform_vector(Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }) - raster_to_camera.transform_vector(Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        let dy_camera: Vector3 = raster_to_camera.transform_vector(Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }) - raster_to_camera.transform_vector(Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        // compute image plane bounds at $z=1$ for _PerspectiveCamera_
        let res: Point2 = film.full_resolution;
        let mut p_min: Point3 = raster_to_camera.transform_point(Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        // Point3 p_max = RasterToCamera(Point3(res.x, res.y, 0));
        let mut p_max: Point3 = raster_to_camera.transform_point(Point3 {
            x: res.x as f64,
            y: res.y as f64,
            z: 0.0,
        });
        p_min /= p_min.z;
        p_max /= p_max.z;
        let a: f64 = ((p_max.x - p_min.x) * (p_max.y - p_min.y)).abs();

        PerspectiveCamera {
            camera_to_world,
            shutter_open,
            shutter_close,
            film:Arc::new(film),
            medium,
            raster_to_camera: Transforms::new(raster_to_camera),
            lens_radius,
            focal_distance,
            dx_camera,
            dy_camera,
            a,
            clipping_start,
        }
    }
    pub fn generate_ray_differential(&self, sample: &CameraSample, ray: &mut Ray) -> f64 {
        let p_film: Point3 = Point3 {
            x: sample.p_film.x,
            y: sample.p_film.y,
            z: 0.0,
        };
        let p_camera: Point3 = self.raster_to_camera.applying_point(p_film);
        let dir: Vector3 = Vector3 {
            x: p_camera.x,
            y: p_camera.y,
            z: p_camera.z,
        }
        .normalize();
        let mut diff: RayDifferential = RayDifferential {
            rx_origin: ray.o,
            ry_origin: ray.o,
            rx_direction: (Vector3 {
                x: p_camera.x,
                y: p_camera.y,
                z: p_camera.z,
            } + self.dx_camera)
                .normalize(),
            ry_direction: (Vector3 {
                x: p_camera.x,
                y: p_camera.y,
                z: p_camera.z,
            } + self.dy_camera)
                .normalize(),
        };
        // *ray = RayDifferential(Point3(0, 0, 0), dir);
        let mut in_ray: Ray = Ray {
            o: Point3::origin(),
            d: dir,
            t_max: std::f64::INFINITY,
            time: sample.time,
            medium: None,
            differential: Some(diff),
        };
        // modify ray for depth of field
        if self.lens_radius > 0.0 as f64 {
            // sample point on lens
            let p_lens: Point2 = Sample::disk_sample_uniform(&sample.p_lens) * self.lens_radius;
            // compute point on plane of focus
            let ft: f64 = self.focal_distance / in_ray.d.z;
            let p_focus: Point3 = in_ray.at(ft);
            // update ray for effect of lens
            in_ray.o = Point3 {
                x: p_lens.x,
                y: p_lens.y,
                z: 0.0 as f64,
            };
            in_ray.d = (p_focus - in_ray.o).normalize();
        }
        // compute offset rays for _PerspectiveCamera_ ray differentials
        if self.lens_radius > 0.0 as f64 {
            // compute _PerspectiveCamera_ ray differentials accounting for lens

            // sample point on lens
            let p_lens: Point2 = Sample::disk_sample_uniform(&sample.p_lens) * self.lens_radius;
            let dx: Vector3 = p_to_v(p_camera + self.dx_camera);
            let ft: f64 = self.focal_distance / dx.z;
            let p_focus: Point3 = Point3::origin() + (dx * ft);
            diff.rx_origin = Point3 {
                x: p_lens.x,
                y: p_lens.y,
                z: 0.0 as f64,
            };
            diff.rx_direction = (p_focus - diff.rx_origin).normalize();
            let dy: Vector3 = p_to_v(p_camera + self.dy_camera).normalize();
            let ft: f64 = self.focal_distance / dy.z;
            let p_focus: Point3 = Point3::origin() + (dy * ft);
            diff.ry_origin = Point3 {
                x: p_lens.x,
                y: p_lens.y,
                z: 0.0 as f64,
            };
            diff.ry_direction = (p_focus - diff.ry_origin).normalize();
            // replace differential
            in_ray.differential = Some(diff);
        }
        // ray->medium = medium;
        if let Some(ref medium_arc) = self.medium {
            in_ray.medium = Some(medium_arc.clone());
        } else {
            in_ray.medium = None;
        }
        *ray = self.camera_to_world.applying_ray(&in_ray);
        1.0
    }
    pub fn we(&self, _ray: &Ray, _p_raster2: Option<&mut Point2>) -> RGBSpectrum {
        // interpolate camera matrix and check if $\w{}$ is forward-facing
        todo!()
    }
    pub fn pdf_we(&self, _ray: &Ray) -> (f64, f64) {
        todo!();
    }
    pub fn sample_wi<'a, 'b>(
        &self,
        _iref: &'a Interaction,
        _lens_intr: &'b mut Interaction,
        _u: Point2,
        _wi: &mut Vector3,
        _pdf: &mut f64,
        _p_raster: &mut Point2,
        // vis: &mut VisibilityTester<'a, 'b>,
    ) -> RGBSpectrum {
        todo!();
        // uniformly sample a lens interaction _lensIntr_
    }
    pub fn get_shutter_open(&self) -> f64 {
        self.shutter_open
    }
    pub fn get_shutter_close(&self) -> f64 {
        self.shutter_close
    }
    pub fn get_film(&self) -> Arc<Film> {
        self.film.clone()
    }
    // ADDED
    pub fn get_clipping_start(&self) -> f64 {
        self.clipping_start
    }
    pub fn adjust_to_clipping_start(&self, _sample: &CameraSample, _ray: &mut Ray) {
        todo!();
    }
    pub fn generate_ray(&self, sample: &CameraSample) -> Ray {
        let p_film: Point3 = Point3 {
            x: sample.p_film.x,
            y: sample.p_film.y,
            z: 0.0,
        };
        let p_camera: Point3 = self.raster_to_camera.applying_point(p_film);
        let mut ray = Ray::default();
        ray.o = Point3::origin();
        ray.d = (p_camera - Point3::origin()).normalize();
        self.camera_to_world.applying_ray(&ray)
    }
}