// see perspective.h

use cgmath::{EuclideanSpace, InnerSpace};

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
    extends::{p_to_v, Point2, Point3, Vector3},
    until::transform::Transforms,
};

pub struct PerspectiveCamera {
    // inherited from Camera (see camera.h)
    pub camera_to_world: Transforms,
    pub shutter_open: f64,
    pub shutter_close: f64,
    pub film: Film,
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
        let camera_to_screen: Transforms = Transforms::perspective(fov, 1e-2, 1000.0);
        // see camera.h
        // compute projective camera screen transformations
        let scale1 = Transforms::scaling(
            film.full_resolution.x as f64,
            film.full_resolution.y as f64,
            1.0,
        );
        let scale2 = Transforms::scaling(
            1.0 / (screen_window.max.x - screen_window.min.x),
            1.0 / (screen_window.min.y - screen_window.max.y),
            1.0,
        );
        let translate = Transforms::translations(-screen_window.min.x, -screen_window.max.y, 0.0);
        let screen_to_raster = scale1 * scale2 * translate;
        let raster_to_screen = Transforms::inverse(&screen_to_raster);
        let raster_to_camera = Transforms::inverse(&camera_to_screen) * raster_to_screen;
        // see perspective.cpp
        // compute differential changes in origin for perspective camera rays
        let dx_camera: Vector3 = raster_to_camera.applying_vector(Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }) - raster_to_camera.applying_vector(Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        let dy_camera: Vector3 = raster_to_camera.applying_vector(Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }) - raster_to_camera.applying_vector(Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        // compute image plane bounds at $z=1$ for _PerspectiveCamera_
        let res: Point2 = film.full_resolution;
        let mut p_min: Point3 = raster_to_camera.applying_point(Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        // Point3 p_max = RasterToCamera(Point3(res.x, res.y, 0));
        let mut p_max: Point3 = raster_to_camera.applying_point(Point3 {
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
            film,
            medium,
            raster_to_camera,
            lens_radius,
            focal_distance,
            dx_camera,
            dy_camera,
            a,
            clipping_start,
        }
    }
    // pub fn create(
    //     params: &ParamSet,
    //     cam2world: AnimatedTransform,
    //     film: Arc<Film>,
    //     medium: Option<Arc<Medium>>,
    //     clipping_start: f64,
    // ) -> Arc<Camera> {
    //     let shutteropen: f64 = params.find_one_float("shutteropen", 0.0);
    //     let shutterclose: f64 = params.find_one_float("shutterclose", 1.0);
    //     // TODO: std::swap(shutterclose, shutteropen);
    //     assert!(shutterclose >= shutteropen);
    //     let lensradius: f64 = params.find_one_float("lensradius", 0.0);
    //     let focaldistance: f64 = params.find_one_float("focaldistance", 1e6);
    //     let frame: f64 = params.find_one_float(
    //         "frameaspectratio",
    //         (film.full_resolution.x as f64) / (film.full_resolution.y as f64),
    //     );
    //     let mut screen: Bounds2f = Bounds2f::default();
    //     if frame > 1.0 {
    //         screen.p_min.x = -frame;
    //         screen.p_max.x = frame;
    //         screen.p_min.y = -1.0;
    //         screen.p_max.y = 1.0;
    //     } else {
    //         screen.p_min.x = -1.0;
    //         screen.p_max.x = 1.0;
    //         screen.p_min.y = -1.0 / frame;
    //         screen.p_max.y = 1.0 / frame;
    //     }
    //     let sw: Vec<f64> = params.find_float("screenwindow");
    //     if !sw.is_empty() && sw.len() == 4 {
    //         screen.p_min.x = sw[0];
    //         screen.p_max.x = sw[1];
    //         screen.p_min.y = sw[2];
    //         screen.p_max.y = sw[3];
    //     }
    //     let fov: f64 = params.find_one_float("fov", 90.0);
    //     // let halffov: f64 =
    //     //     params.find_one_float(String::from("halffov"), -1.0);
    //     // TODO: if (halffov > 0.f)
    //     // TODO: let perspective_camera: Arc<Camera + Sync + Send> =
    //     Arc::new(Camera::Perspective(Box::new(PerspectiveCamera::new(
    //         cam2world,
    //         screen,
    //         shutteropen,
    //         shutterclose,
    //         lensradius,
    //         focaldistance,
    //         fov,
    //         film,
    //         medium,
    //         clipping_start,
    //     ))))
    // }
    // Camera
    pub fn generate_ray_differential(&self, sample: &CameraSample, ray: &mut Ray) -> f64 {
        // TODO: ProfilePhase prof(Prof::GenerateCameraRay);
        // compute raster and camera sample positions
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
    pub fn get_film(&self) -> Film {
        todo!();
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
        ray.d = (p_camera - ray.o).normalize();
        self.camera_to_world.applying_ray(&ray)
    }
}
#[cfg(test)]
mod test {
    use cgmath::EuclideanSpace;

    use crate::{
        core::{
            aabb::Bounds3,
            film::{self, Film},
            sample::CameraSample,
        },
        until::transform::Transforms,
    };

    use super::PerspectiveCamera;
    use crate::extends::*;

    #[test]
    fn test_perspective() {
        let mut  film = Film::new(Point2::new(200.0, 200.0), "test_camera.png");
        let camera = PerspectiveCamera::new(
            Transforms::look_at_lh(
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, 1.0),
                Vector3::unit_y(),
            ),
            Bounds3::new(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 0.0)),
            0.0,
            0.0,
            0.0,
            1.0,
            90.0,
            film,
            None,
            0.0,
        );
        let sample = CameraSample::new(Point2::new(1.0, 1.0), 0.0);
        {

        }
        let ray = camera.generate_ray(&sample);
        println!("{:?}", ray)
    }
}
