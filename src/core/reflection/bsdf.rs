use cgmath::InnerSpace;

use crate::{core::interaction::SurfaceInteraction, extends::Vector3};

use super::Bxdf;

///BSDF类代表了BRDFs和BTDFs的集合。
/// 以这种方式分组，可以让系统的其他部分直接与复合BSDFs一起工作，
/// 而不必考虑它们可能是由所有组件构成的。
/// 同样重要的是，BSDF类向系统的其他部分隐藏了阴影法线的一些细节。
/// 着色法线，无论是来自三角形网格中的每顶点法线，还是来自凹凸贴图，都可以极大地提高渲染场景的视觉丰富度，
/// 但是由于它们是一种特殊的构造，要将它们纳入基于物理的渲染器中是很困难的。
/// 它们所带来的问题在BSDF的实现中得到了处理。
const MAX_BXDFS :usize=20;
#[derive(Debug, Clone)]
pub struct BSDF {
    //相对折射率，不允许折射是传递1
    pub eta: f64,
    //渲染法线
    pub normal_s: Vector3,
    //几何法线
    pub normal_g: Vector3,
    pub ss: Vector3,
    pub ts: Vector3,
    pub bxdfs: Vec<Bxdf>,
}
impl BSDF {
    pub fn new(surface: &SurfaceInteraction, eta: f64) -> Self {
        let ss = surface.shading.dpdu.normalize();
        BSDF {
            eta ,
            normal_s: surface.shading.n,
            normal_g: surface.normal,
            ss,
            ts:surface.shading.n.cross(ss),
            bxdfs: Vec::with_capacity(8),
        }
    }
    pub fn add(&mut self,b:Bxdf){
        assert!(self.bxdfs.len()<MAX_BXDFS);
        self.bxdfs.push(b);
    }
    pub fn world_to_local(&self,v:&Vector3)->Vector3{
        Vector3::new(
            v.dot(self.ss),
            v.dot(self.ts),
            v.dot(self.normal_s)
        )
    }
    pub fn local_to_world(&self,v:&Vector3)->Vector3{
        Vector3::new(
            self.ss.x*v.x+self.ts.x+v.y+self.normal_s.x*v.z,
            self.ss.y*v.x+self.ts.y+v.y+self.normal_s.y*v.z,
            self.ss.z*v.x+self.ts.z+v.y+self.normal_s.z*v.z,

        )
    }
}
