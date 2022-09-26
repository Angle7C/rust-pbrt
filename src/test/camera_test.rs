#[cfg(test)]
mod test {
    use crate::{
        core::{
            aabb::Bounds2,
            camera::{perspecttivecamera::PerspectiveCamera, CameraAble},
            film::Film,
            *,
        },
        extends::*,
    };
    #[test]
    fn test_camera() {
        let film = Film::new(Vec2::new(100.0, 100.0), "test_camear");
        let sampler = sample::CameraSample::new(Point2::new(0.0, 0.0), 1.0);
        let camera = PerspectiveCamera::new(
            Mat4::IDENTITY,
            Bounds2::new(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0)),
            0.0,
            1.0,
            1.0,
            1.0,
            90.0,
            &film,
            None,
        );
        let (_,_) = camera.generate_ray(&sampler);
        
    }
}
