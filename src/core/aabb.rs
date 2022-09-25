use std::mem::swap;

use crate::extends::{Point2, Point3, Vec2, Vec3};

use super::ray::Ray;
pub struct Bounds3 {
    pub min: Point3,
    pub max: Point3,
}
impl Bounds3 {
    ///包围盒初始化
    pub fn init() -> Self {
        Self {
            min: (Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY)),
            max: (Point3::new(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY)),
        }
    }
    ///将包围盒初始化到一个点上
    pub fn init_point(point: Point3) -> Self {
        Self {
            min: (point),
            max: (point),
        }
    }
    ///将包围盒初始化到Min，Max包围的空间
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min: min, max: max }
    }
    ///将两个包围盒合并到一起
    pub fn union_bound(&self, other: &Bounds3) -> Self {
        let min = Point3::new(
            self.min.x.min(other.min.x),
            self.min.y.min(other.min.y),
            self.min.z.min(other.min.z),
        );
        let max = Point3::new(
            self.max.x.max(other.max.x),
            self.max.y.max(other.max.y),
            self.max.z.max(other.max.z),
        );
        Self { min: min, max: max }
    }
    ///将包围盒与一个点合并
    pub fn union_point(&self, p: Point3) -> Self {
        let min = Point3::new(
            self.min.x.min(p.x),
            self.min.y.min(p.y),
            self.min.z.min(p.z),
        );
        let max = Point3::new(
            self.max.x.max(p.x),
            self.max.y.max(p.y),
            self.max.z.max(p.z),
        );
        Self {
            min: (min),
            max: (max),
        }
    }
    ///求两个包围盒的交集
    pub fn intersect(&self, other: &Bounds3) -> Self {
        let min = Point3::new(
            self.min.x.max(other.min.x),
            self.min.y.max(other.min.y),
            self.min.z.max(other.min.z),
        );
        let max = Point3::new(
            self.max.x.min(other.max.x),
            self.max.y.min(other.max.y),
            self.max.z.min(other.max.z),
        );
        Self {
            min: (min),
            max: (max),
        }
    }
    ///求包围盒中是否存在一个点
    pub fn inside(&self, p: &Point3) -> bool {
        return p.x >= self.min.x
            && p.x <= self.max.x
            && p.y >= self.min.y
            && p.y <= self.max.y
            && p.z >= self.min.z
            && p.z <= self.max.z;
    }
    ///扩张包围的边界det
    pub fn expend(&self, det: Point3) -> Self {
        Self::new(self.min - det, self.max + det)
    }
    ///求包围盒的对角线向量
    pub fn diagonal(&self) -> Vec3 {
        self.max - self.min
    }
    ///对包围盒进行插值
    pub fn lerp(&self) -> Self {
        todo!()
    }
    ///未定义，请勿使用
    pub fn offset(&self, _p: &Point3) -> Vec3 {
        todo!()
    }
    //求包围盒的外接球
    pub fn bound_sphere(&self) -> Self {
        todo!()
    }
    #[allow(unused_comparisons)]
    ///求出包围盒的第i点
    pub fn rang_point(&self, i: usize) -> Vec3 {
        if i > 8 || i < 0 {
            panic!("the index is out")
        }
        let x = if i & 1 == 0 { self.min.x } else { self.max.x };
        let y = if i & 2 == 0 { self.min.y } else { self.max.y };
        let z = if i & 4 == 0 { self.min.z } else { self.max.z };
        Vec3::new(x, y, z)
    }
    ///求包围盒的面积
    pub fn area(&self) -> f32 {
        let det = self.max - self.min;
        det.x.abs() * det.y.abs() * det.z.abs() * 2.0
    }
    ///求光线是否与包围盒有交点
    #[inline]
    pub fn intersect_ray(&self, ray: &Ray) -> (bool,f32,f32) {
        let inv_dir = 1.0 / ray.d;
        let mut t0 = 0.0;
        let mut t1 = ray.t_max;
        for i in 0..3 {
            let mut near = (self.max[i] - ray.o[i]) / inv_dir[i];
            let mut  far = (self.min[i] - ray.o[i]) / inv_dir[i];
            if near > far {
                swap(&mut near, &mut far);
            };
            t0 = if near > t0 { near } else { t0 };
            t1 = if far < t1 { far } else { t1 };
            if t0 > t1 {
                return (false,-1.0,-1.0)
            }
        };
        return (true,t0,t1)
    }

}

pub struct Bounds2 {
    pub min: Point2,
    pub max: Point2,
}
impl Bounds2 {
    pub fn area(&self) -> f32 {
        let det = self.max - self.min;
        det.x.abs() * det.y.abs()
    }
    pub fn init() -> Self {
        Self {
            min: (Point2::new(f32::INFINITY, f32::INFINITY)),
            max: (Point2::new(-f32::INFINITY, -f32::INFINITY)),
        }
    }
    pub fn init_point(point: Point2) -> Self {
        Self {
            min: (point),
            max: (point),
        }
    }
    pub fn new(min: Point2, max: Point2) -> Self {
        Self { min: min, max: max }
    }
    pub fn union_bound(&self, other: &Bounds3) -> Self {
        let min = Point2::new(
            self.min.x.min(other.min.x),
            self.min.y.min(other.min.y),
            // self.min.z.min(other.min.z),
        );
        let max = Point2::new(
            self.max.x.max(other.max.x),
            self.max.y.max(other.max.y),
            // self.max.z.max(other.max.z),
        );
        Self { min: min, max: max }
    }
    pub fn union_point(&self, p: Point2) -> Self {
        let min = Point2::new(
            self.min.x.min(p.x),
            self.min.y.min(p.y),
            // self.min.z.min(p.z),
        );
        let max = Point2::new(
            self.max.x.max(p.x),
            self.max.y.max(p.y),
            // self.max.z.max(p.z),
        );
        Self {
            min: (min),
            max: (max),
        }
    }
    pub fn intersect(&self, other: &Bounds2) -> Self {
        let min = Point2::new(
            self.min.x.max(other.min.x),
            self.min.y.max(other.min.y),
            // self.min.z.max(other.min.z),
        );
        let max = Point2::new(
            self.max.x.min(other.max.x),
            self.max.y.min(other.max.y),
            // self.max.z.min(other.max.z),
        );
        Self {
            min: (min),
            max: (max),
        }
    }
    pub fn inside(&self, p: &Point2) -> bool {
        return p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y;
        // && p.z >= self.min.z
        // && p.z <= self.max.z;
    }
    pub fn expend(&self, det: Point2) -> Self {
        Self::new(self.min - det, self.max + det)
    }
    pub fn diagonal(&self) -> Vec2 {
        self.max - self.min
    }
    pub fn lerp(&self) -> Self {
        todo!()
    }
    pub fn offset(&self, _p: &Point2) -> Vec2 {
        todo!()
    }
    pub fn bound_sphere(&self) -> Self {
        todo!()
    }
    #[allow(unused_comparisons)]
    pub fn rang_point(&self, i: usize) -> Vec2 {
        if i > 8 || i < 0 {
            panic!("the index is out")
        }
        let x = if i & 1 == 0 { self.min.x } else { self.max.x };
        let y = if i & 2 == 0 { self.min.y } else { self.max.y };
        // let z = if i & 4 == 0 { self.min.z } else { self.max.z };
        Vec2::new(x, y)
    }
}
#[cfg(test)]
mod test {
    use super::Bounds3;
    use crate::extends::Vec3;
    #[test]
    pub fn get_bound() {
        let bound = Bounds3::new(Vec3::new(1.0, 3.0, 5.0), Vec3::new(2.0, 4.0, 6.0));
        for i in 0..8 {
            println!("{:?}", bound.rang_point(i));
        }
    }
}
