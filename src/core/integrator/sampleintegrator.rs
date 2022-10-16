use std::sync::Arc;

use crate::core::{
    camera::Camera, interaction::SurfaceInteraction, ray::{RayDifferential, Ray}, sample::Sampler,
    scene::Scene, spectrum::RGBSpectrum, aabb::Bounds3, light::LightDistribution,
};

pub struct PathIntegrator {
    pub sampler: Box<Sampler>,
    pub camera: Arc<Camera>,
    pub pixel_bounds:Bounds3,
    pub max_depth:u32,
    pub rr_threshold:f64,
    //光源采样策略
    pub light_sample_strategy:String,
    pub light_distribution:Option<Arc<LightDistribution>>
}
impl PathIntegrator {
    pub fn new(
        max_depth: u32,
        camera: Arc<Camera>,
        sampler: Box<Sampler>,
        pixel_bounds: Bounds3,
        rr_threshold: f64,
        
        light_sample_strategy: String,
    ) -> Self {
        
        PathIntegrator {
            camera,
            sampler,
            pixel_bounds,
            max_depth,
            rr_threshold,
            light_distribution: None,
            light_sample_strategy
        }
    }
    pub fn preprocess(_scene: &Scene, _sample: &Sampler) {
        unimplemented!()
    }
    pub fn li(
        ray: &Ray,
        scene: &Scene,
        _sample: &Sampler,
        _depth: u32,
    ) -> RGBSpectrum {
        // /求解场景与光线的交点，
        // / 是否超过光的最大传播时间，迭代深度。
        // /计算介质散射
        // /从光源中采样
        // / 获取新路径的BSDF
        // / 计算次表面散射
        // / 使用俄罗斯轮盘来赌
        // let mut l=RGBSpectrum::default();
        // let mut beta=RGBSpectrum::from_value(1.0);
        loop{
            let mut isect=SurfaceInteraction::default();
            if scene.intersect(ray, &mut isect){
                
            }
        }
    }
    pub fn specular_reflect(
        _ray: &RayDifferential,
        _isect: &SurfaceInteraction,
        _scene: &Scene,
        _sample: &Sampler,
        _depth: u32,
    ) -> RGBSpectrum {
        unimplemented!()
    }
    pub fn specular_transmit(
        _ray: &RayDifferential,
        _isect: &SurfaceInteraction,
        _scene: &Scene,
        _sample: &Sampler,
        _depth: u32,
    ) -> RGBSpectrum {
        unimplemented!()
    }
    pub fn get_camera(&self)->Arc<Camera>{
        self.camera.clone()
    }
    pub fn get_sampler(&self)->&Sampler{
        &self.sampler
    }
}
