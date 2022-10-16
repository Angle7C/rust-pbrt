use std::rc::Rc;
use std::sync::Arc;
use cgmath::MetricSpace;

use crate::extends::{ Point3};
use crate::core::bxdf::BxdF;
use self::bvh::BVH;

use super::{
    aabb::Bounds3,
   
    interaction::{ SurfaceInteraction},
    light::Light,
    ray::Ray,
    shape::{ Shape}, material::Material,
};
pub mod bvh;
pub struct Primitive {
    //shape索引
    pub shape_index: usize,
    //材质
    pub material: Option<Arc<Material>>,
    //光源
    pub light: Option<Arc<Light>>,
    //反射模型
    pub bxdf:  Option<BxdF>,
}
impl Primitive {
    pub fn new(
        shape_index: usize,
        bxdf: Option< BxdF>,
        materail: Option<Arc<Material>>,
        light: Option<Arc<Light>>,
    ) -> Self {
        Self {
            shape_index: (shape_index),
            material: (materail),
            light: (light),
            bxdf: bxdf,
        }
    }
    pub fn get_bound(&self,shape:&Vec<Shape>)->Bounds3{
        shape[self.shape_index].object_world_bound()
    }
    pub fn get_center(&self,shape:&Vec<Shape>)->Point3{
        shape[self.shape_index].object_world_bound().center()
        
    }
    pub fn world_bounds(&self,shape:&Vec<Shape>) -> Bounds3{
        shape[self.shape_index].object_world_bound()
    }
    pub fn intersect_p(&self, ray: &Ray,shape:&Vec<Shape>) -> bool{
        shape[self.shape_index].intersect_p(ray)
    }
    pub fn intersect(&self, ray: &mut Ray,shape:&Vec<Shape>,isect:&mut SurfaceInteraction) -> bool{
        shape[self.shape_index].intersect(ray,isect)
    }
    pub fn get_area_light(&self) -> Option<&Rc<Light>>{
        unimplemented!()
    }
    pub fn get_material(&self) -> Option<&Rc<Material>>{
        unimplemented!()
    }
}
impl Default for Primitive {
    fn default() -> Self {
        Self::new(0,None, None, None)
    }
    
}
pub struct Aggregate<'a>{
    primitive:Arc<Vec<Primitive>>,
    bvh:BVH<'a>,
}
impl<'a> Default for Aggregate<'a>{
    fn default() -> Self {
        Self { primitive: Arc::new(vec![]), bvh: (BVH::default()) }
    }
}
impl<'a> Aggregate<'a>{
    pub fn new<'b>(list :&'b Vec<Shape>)->Self
    where 'b:'a
    {    
            let mut t :Vec<_>=list.iter().enumerate().map(|(i,_)|{
                let mut p = Primitive::default();
                p.shape_index=i;
                p
            }).collect();
            Self { 
                bvh:BVH::build(&mut t, &list),
                primitive:Arc::new(t),
             }
    }
    pub fn hit(&self,ray:&Ray,isect:&mut SurfaceInteraction)->bool{
        if let Some(v)=self.bvh.hit_BVH(&ray, &self.primitive){
            let mut judge=false;
            let mut min_dis=f64::MAX;
            // let mut temp=SurfaceInteraction::init();
            match self.bvh.shape_message {
                None=>false,
                Some(shapes)=>{
                    v.iter().for_each(|&index|{
                        let dis =shapes[index].object_world_bound().center().distance(ray.o);
                        //没有击中shape或者击中的shape距离o点的距离小于之前击中点的距离。
                        if min_dis>dis||judge==false{
                            //击中才去更新isect
                            judge=shapes[index].intersect(&ray,isect);
                            if judge==true{
                                min_dis=dis;
                            }                    
                        }
                    });
                    judge
                }
            }
        }else{
            false
        }

        
    }
}