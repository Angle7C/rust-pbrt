
use PMXUtil::types::{Vertex, Face};
use bvh::{aabb::{Bounded, AABB}, Point3, bounding_hierarchy::BHShape};
use euler::Vec3;

pub struct Triangle{
    pub pos:[Vec3;3],
    pub index :usize
}
impl Bounded for Triangle{
    fn aabb(&self) -> bvh::aabb::AABB {
        let mut max:Point3=Point3::new(-f32::INFINITY,-f32::INFINITY,-f32::INFINITY);
        let mut min:Point3=Point3::new(f32::INFINITY,f32::INFINITY,f32::INFINITY);
        self.pos.iter().for_each(|item|{
            max.x=max.x.max(item.x+0.02);
            max.y=max.y.max(item.y+0.02);
            max.z=max.z.max(item.z+0.02);
            min.x=min.x.min(item.x-0.02);
            min.y=min.x.min(item.y-0.02);
            min.z=min.x.min(item.z-0.02);
        });
        AABB::with_bounds(min, max)
    }
}
impl BHShape for Triangle{
    fn bh_node_index(&self) -> usize {
        self.index
    }
    fn set_bh_node_index(&mut self, index: usize) {
        self.index=index
    }
}
impl Triangle {
    pub fn new(faces:&Face,postion :&Vec<Vertex>)->Self{
        let (a,b,c)=(faces.vertices[0],faces.vertices[1],faces.vertices[2]);
        let a=Vec3::new(postion[a as usize].position[0],postion[a as usize].position[1],postion[a as usize].position[2]);  
        let b=Vec3::new(postion[b as usize].position[0],postion[b as usize].position[1],postion[b as usize].position[2]);  
        let c=Vec3::new(postion[c as usize].position[0],postion[c as usize].position[1],postion[c as usize].position[2]); 
        Self{
            pos:[a,b,c],
            index:0
        }
    }
}