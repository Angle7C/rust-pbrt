use crate::{
    core::{
        aabb::Bounds3,
        ray::{ Ray}, shape::Shape,
    },
    
};

use super::Primitive;

pub struct BVH<'a> {
    pub nodelist: Box<Vec<BVHNode>>,
    pub shape_message: &'a Vec<Shape>,
}
#[derive(Clone, Copy)]

struct Bucket {
    pub size: f64,
    pub aabb: Bounds3,
}
impl Bucket {
    const BUCKECTSIZE: usize = 6;
    pub fn add_aabb(&mut self, aabb: &Bounds3) {
      self.size += 1.0;
       self.aabb=self.aabb.union_bound(&aabb);
    }
    pub fn add_bucket(a: Self, other: &Bucket) -> Bucket {
        Self {
            size: a.size + other.size,
            aabb: a.aabb.union_bound(&other.aabb),
        }
    }
    pub fn init()->Self{
        Self { size: (0.0), aabb: (Bounds3::default()) }
    }
}
impl Default for Bucket {
    fn default() -> Self {
        Self {
            size: (0.0),
            aabb: (Bounds3::default()),
        }
    }
}
pub enum BVHNode {
    Leaf {
        parent_index: usize,
        shape_index: usize,
        depth: usize,
        aabb: Bounds3,
    },
    Node {
        l_index: usize,
        r_index: usize,
        depth: usize,
        parent_index: usize,
        l_aabb: Bounds3,
        r_aabb: Bounds3,
    },
}
impl Default for BVHNode {
    fn default() -> Self {
        BVHNode::Node {
            l_index: (0),
            r_index: (0),
            depth: (0),
            parent_index: 0,
            l_aabb: Bounds3::default(),
            r_aabb: Bounds3::default(),
        }
    }
}
impl<'a> BVHNode {
    #[allow(dead_code)]
    const BUCKECTSIZE: usize = 6;
    pub fn build(
        shape: &mut [Primitive],
        shape_message: &'a Vec<Shape>,
        indices: &[usize],
        nodes: &mut Vec<BVHNode>,
        parent: usize,
        depth: usize,
    ) -> usize {
    
        let message = {
            let mut message = Bounds3::default();
            for index in indices {
                message=message.union_bound(&shape[*index].get_bound(shape_message));
            }
            message
        };
        if indices.len() == 1 {
            nodes.push(BVHNode::Leaf {
                parent_index: parent,
                shape_index: indices[0],
                depth: depth,
                aabb: shape[indices[0]].get_bound(shape_message),
            });
            return nodes.len()-1;
        }
        
        let axis = message.max_axis();
        let axis_size = message.diagonal()[axis];
        let node_index = nodes.len();
        nodes.push(BVHNode::default());
        let (l_aabb, r_aabb, l_index, r_index) = if indices.len() == 2 {
            let (l_index, r_index) = indices.split_at(indices.len() / 2);
       
            let l_aabb = get_aabb(l_index, &shape, shape_message);
            let r_aabb = get_aabb(r_index, &shape, shape_message);
            if l_index.len()==0{
                panic!("l is zero");
            }
            if r_index.len()==0{
                panic!("r is zero");

            }
            let l = BVHNode::build(shape, shape_message, l_index, nodes, parent, depth + 1);
            let r = BVHNode::build(shape, shape_message, r_index, nodes, parent, depth + 1);
            (l_aabb, r_aabb, l, r)
        } else {
            let mut buckets = [Bucket::default(); Bucket::BUCKECTSIZE];
            let mut buckets_indices: [Vec<usize>; Bucket::BUCKECTSIZE] = Default::default();
            //将所有AABB平均装到BUCKETSIZE个桶里
            for index in indices {
                let t = &shape_message[shape[*index].shape_index];
                let aabb = &t.object_world_bound();
                let center = &aabb.center();
                let bucket_index = (center[axis] - message.min[axis]) / axis_size;
                let bucket_index = (bucket_index * (Bucket::BUCKECTSIZE as f64 - 0.01)) as usize;
                buckets[bucket_index].add_aabb(&aabb);
                buckets_indices[bucket_index].push(*index);
            }
            //计算最小花费
            let mut cost = 1200_2000_200.0;
            let mut min_index = 0;
            let mut l_aabb = Bounds3::default();
            let mut r_aabb = Bounds3::default();
            for i in 0..(Bucket::BUCKECTSIZE - 1) {
                let (l_bucket, r_bucket) = buckets.split_at(i + 1);
                let mut l_b=Bucket::init();
                 l_bucket.iter().for_each(|x|{
                   l_b=Bucket::add_bucket(l_b, x);
                });
                let mut r=Bucket::default();
                 r_bucket.iter().for_each(|x|{
                    r=Bucket::add_bucket(r, x);
                });
                let cost_t = (l_b.size * l_b.aabb.area() + r.size * r.aabb.area()) / message.area();
                if cost > cost_t {
                    cost = cost_t;
                    min_index = i;
                    l_aabb = l_b.aabb;
                    r_aabb = r.aabb;
                }
            }
            let (l_indices, r_indices) = buckets_indices.split_at_mut(min_index + 1);
            let l_indices = vector_move_new(l_indices);
            let r_indices = vector_move_new(r_indices);
            let l_child =
                BVHNode::build(shape, shape_message, &l_indices, nodes, node_index, depth + 1);
            let r_child =
                BVHNode::build(shape, shape_message, &r_indices, nodes, node_index, depth + 1);
            (l_aabb, r_aabb, l_child, r_child)
        };
        nodes[node_index] = BVHNode::Node {
            l_index: l_index,
            r_index: r_index,
            depth: depth,
            parent_index: parent,
            l_aabb: l_aabb,
            r_aabb: r_aabb,
        };
        node_index
    }
    pub fn hit(
        nodes: &Vec<BVHNode>,
        ray: &Ray,
        shape: &Vec<Primitive>,
        shape_message: &'a Vec<Shape>,
        ans: &mut Vec<usize>,
        node_index: usize,
    ) {
        match nodes[node_index] {
            Self::Node {
                l_index,
                r_index,
                ref l_aabb,
                ref r_aabb,
                ..
            } => {
                if let (true, _, _) = l_aabb.intersect_ray(ray) {
                    Self::hit(nodes, ray, shape, shape_message, ans, l_index)
                }
                if let (true, _, _) = r_aabb.intersect_ray(ray) {
                    Self::hit(nodes, ray, shape, shape_message, ans, r_index)
                }
            }
            Self::Leaf {
                shape_index, aabb, ..
            } => {
                if let (true, _, _) = aabb.intersect_ray(ray) {
                    ans.push(shape_index)
                }
            }
        }
    }
}
impl PartialEq for BVHNode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                BVHNode::Leaf {
                    parent_index: parent_index_l,
                    shape_index: shape_index_l,
                    depth: depth_l,
                    ..
                },
                BVHNode::Leaf {
                    parent_index: parent_index_r,
                    shape_index: shape_index_r,
                    depth: depth_r,
                    ..
                },
            ) => {
                parent_index_l == parent_index_r
                    && shape_index_l == shape_index_r
                    && depth_l == depth_r
            }
            (
                BVHNode::Node {
                    l_index: l1,
                    r_index: r1,
                    depth: d1,
                    parent_index: p1,
                    ..
                },
                BVHNode::Node {
                    l_index: l2,
                    r_index: r2,
                    depth: d2,
                    parent_index: p2,
                    ..
                },
            ) => l1 == l2 && r1 == r2 && d1 == d2 && p1 == p2,
            _ => false,
        }
    }
    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}
impl<'a> BVH<'a> {
    pub fn build<'b>(
        shape: &mut Vec<Primitive>,
        shape_message: &'b Vec<Shape>,
    ) -> Self
    where
        'b: 'a,
    {
        let indices = (0..shape.len()).collect::<Vec<usize>>();
        let mut nodes = Box::new(Vec::<BVHNode>::with_capacity(2 * shape.len()));
        BVHNode::build(shape, shape_message, &indices, &mut nodes, 0, 0);
        Self {
            nodelist: (nodes),
            shape_message: (shape_message),
        }
    }
    pub fn hit_BVH(&self, ray: &Ray, shape: &Vec<Primitive>) -> Vec<usize> {
        let mut ans = Vec::new();
        BVHNode::hit(&self.nodelist, ray, shape, self.shape_message, &mut ans, 0);
        ans
    }
}
fn get_aabb(
    list: &[usize],
    shape: &[Primitive],
    shape_message: &Vec<Shape>,
) -> Bounds3 {
    let aabb = Bounds3::default();
    for index in list {
        shape[*index].get_bound(shape_message);
    }
    aabb
}
fn vector_move_new(vec: &mut [Vec<usize>]) -> Vec<usize> {
    let mut t = Vec::new();
    for i in vec.iter_mut() {
        t.append(i);
    }
    t
}
#[cfg(test)]
mod test {
   
    }
