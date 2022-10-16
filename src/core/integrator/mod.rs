use std::sync::Arc;

use crate::{extends::Point2, until::block::Block};

use self::sampleintegrator::PathIntegrator;

use super::{
    camera::Camera, film::FilmTile, ray::Ray, sample::Sampler, scene::Scene, spectrum::RGBSpectrum, aabb::Bound2,
};

pub mod sampleintegrator;
pub enum Integrator {
    Path(PathIntegrator),
}
unsafe impl Send for Integrator{}
//分片大小
const TILE_SIZE: u32 = 16;
impl Integrator {
    pub fn render(&self, scene: &Scene, num_threads: u8) {
        /*

           获得胶片
           获得像素范围
           计算有多少分块
           计算线程核心数
           使用阻塞队列，来获得单个像素。
           获取到相机，胶片，采样器，阻塞队列，积分器的多线程版本
           开启线程
           {
           获取胶片的该线程的渲染分块
            while{
              采样器重置
              获得一个像素
              while{
               对该像素进行超采样
               while{
                   获得相机采样
                   通过相机采样生成微分光线。
                   对微分光线进行缩放。
                   根据光线权重来求解渲染方程
                   检查解的合法性
                   保存采样的值
               }
               保存像素值。
              }
            }

           }
           输出图片
        */
        let camera = self.get_camera();
        let film = camera.get_film();
        
        let bounds = film.full_resolution;
        let cores = if num_threads == 0 { 8 } else { num_threads };
        
        let block = Block::new((bounds.x as u32, bounds.y as u32), (TILE_SIZE, TILE_SIZE));
        let bq = &block;
        {
            let &integrator=&self;
            let film=&film;
            let camera=&camera;
            let bound = &bounds;
            let sampler = &self.get_sampler();
            crossbeam::scope(|scope| {
                let (send, revice) = crossbeam::channel::bounded::<FilmTile>(cores as usize);
                for _ in 0..cores {
                    let send = send.clone();
                    let mut tile_sampler = sampler.clone_with_seed();
                    scope.spawn(move |_| {
                        while let Some((x, y)) = bq.next() {
                            let x0 = x * TILE_SIZE;
                            let x1 = (x * TILE_SIZE + TILE_SIZE).min(bound.x as u32);
                            let y0 = y * TILE_SIZE;
                            let y1 = (y * TILE_SIZE + TILE_SIZE).min(bound.y as u32);
                            let bound2 = Bound2::new(
                                Point2::new(x0 as f64, y0 as f64),
                                Point2::new(x1 as f64, y1 as f64),
                            );
                            let mut tile = film.get_film_tile(&bound2);
                            tile_sampler.reseed((x1*y1+x0) as  u64 );
                            for pixel in bound2.into_iter(){
                                // let min = pixel;
                                tile_sampler.start_pixel(pixel);
                                loop {
                                    let camera_sampler = tile_sampler.get_camera_sample(pixel);

                                    let (mut color, weight) = match camera
                                        .generater_ray_differential(&camera_sampler)
                                    {
                                        (mut ray, weight) => {
                                            if weight!=0.0{
                                                (RGBSpectrum::default(), 0.0)
                                            }else{
                                                (integrator.li(&mut ray, scene, &mut tile_sampler, 0), weight)

                                            }
                                        }
                                    };
                                    tile.add_sample(camera_sampler.p_film, &mut color, weight);
                                    if !tile_sampler.start_next_sample() {
                                        break;
                                    }
                                }
                            }
                            send.send(tile)
                                .unwrap_or_else(|_| panic!("tile send is faiule"));
                        }
                    });
                }
                scope.spawn(move |_| {
                    for item in revice {
                        film.merge_film_tile(&item);
                    }
                });
            })
            .unwrap();
        };
    }
    pub fn process(&self, _scene: &Scene) {}
    pub fn get_sampler(&self) -> &Sampler {
        match self {
            Self::Path(v)=>v.get_sampler(),
            // _=>unimplemented!(),
        }
    }
    pub fn get_camera(&self) -> Arc<Camera> {
        match self {
            Self::Path(v) => v.get_camera(),
            // _ => todo!(),
        }
    }
    pub fn li(
        &self,
        _ray: &mut Ray,
        _scene: &Scene,
        _sampler: &mut Sampler,
        _depth: u32,
    ) -> RGBSpectrum {
        unimplemented!()
    }
}
