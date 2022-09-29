use std::{mem::swap, ops::Deref};

use crate::extends::{Point3, Vector3};

use super::ray::Ray;
#[derive(Clone, Debug,Copy)]
pub struct Bounds3 {
    pub min: Point3,
    pub max: Point3,
}
impl Default for Bounds3 {
    fn default() -> Self {
        Self::init()
    }
}
impl Bounds3 {
    ///包围盒初始化
    pub fn init() -> Self {
        Self {
            min: (Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY)),
            max: (Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY)),
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
            self.min[0].min(other.min[0]),
            self.min[1].min(other.min[1]),
            self.min[2].min(other.min[2]),
        );
        let max = Point3::new(
            self.max[0].max(other.max[0]),
            self.max[1].max(other.max[1]),
            self.max[2].max(other.max[2]),
        );
        Self { min: min, max: max }
    }
    ///将包围盒与一个点合并
    pub fn union_point(&self, p: Point3) -> Self {
        let min = Point3::new(
            self.min[0].min(p[0]),
            self.min[1].min(p[1]),
            self.min[2].min(p[2]),
        );
        let max = Point3::new(
            self.max[0].max(p[0]),
            self.max[1].max(p[1]),
            self.max[2].max(p[2]),
        );
        Self {
            min: (min),
            max: (max),
        }
    }
    ///求两个包围盒的交集
    pub fn intersect(&self, other: &Bounds3) -> Self {
        let min = Point3::new(
            self.min[0].max(other.min[0]),
            self.min[1].max(other.min[1]),
            self.min[2].max(other.min[2]),
        );
        let max = Point3::new(
            self.max[0].min(other.max[0]),
            self.max[1].min(other.max[1]),
            self.max[2].min(other.max[2]),
        );
        Self {
            min: (min),
            max: (max),
        }
    }
    ///求包围盒中是否存在一个点
    pub fn inside(&self, p: &Point3) -> bool {
        return p[0] >= self.min[0]
            && p[0] <= self.max[0]
            && p[1] >= self.min[1]
            && p[1] <= self.max[1]
            && p[2] >= self.min[2]
            && p[2] <= self.max[2];
    }
    ///扩张包围的边界det
    pub fn expend(&self, det: Point3) -> Self {
        let x=self.min.x-det.x;
        let y=self.min.y-det.y;
        let z=self.min.z-det.z;
        let min=Point3::new(x, y, z);

        let x=self.min.x+det.x;
        let y=self.min.y+det.y;
        let z=self.min.z+det.z;
        let max=Point3::new(x, y, z);
        Self { min: min, max: max }
    }
    ///求包围盒的对角线向量
    pub fn diagonal(&self) -> Vector3 {
        self.max - self.min
    }
    pub fn center(&self) -> Point3 {
        let x=(self.min.x+self.max.x)/2.0;
        let y=(self.min.y+self.max.y)/2.0;
        let z=(self.min.z+self.max.z)/2.0;
        Point3::new(x, y, z)

    }
    ///对包围盒进行插值
    pub fn lerp(&self) -> Self {
        todo!()
    }
    ///未定义，请勿使用
    pub fn offset(&self, _p: &Point3) -> Vector3 {
        todo!()
    }
    //求包围盒的外接球
    pub fn bound_sphere(&self) -> Self {
        todo!()
    }
    #[allow(unused_comparisons)]
    ///求出包围盒的第i点
    pub fn rang_point(&self, i: usize) -> Point3 {
        if i > 8 || i < 0 {
            panic!("the index is out")
        }
        let x = if i & 1 == 0 { self.min[0] } else { self.max[0] };
        let y = if i & 2 == 0 { self.min[1] } else { self.max[1] };
        let z = if i & 4 == 0 { self.min[2] } else { self.max[2] };
        Point3::new(x, y, z)
    }
    ///求包围盒的面积
    pub fn area(&self) -> f64 {
        let det = self.max - self.min;
        det[0].abs() * det[1].abs() * det[2].abs()
    }
    pub fn max_axis(&self) -> usize {
        let det = self.max - self.min;
        if det[0] > det[1] && det[0] > det[2] {
            0
        } else if det[1] > det[0] && det[1] > det[2] {
            1
        } else {
            2
        }
    }
    ///求光线是否与包围盒有交点
    #[inline]
    pub fn intersect_ray(&self, ray: &Ray) -> (bool, f64, f64) {
        let inv_dir = 1.0 / ray.d;
        let mut t0 = 0.0;
        let mut t1 = ray.t_max;
        for i in 0..3 {
            let mut near = (self.max[i] - ray.o[i]) * inv_dir[i];
            let mut far = (self.min[i] - ray.o[i]) * inv_dir[i];
            if near > far {
                swap(&mut near, &mut far);
            };
            t0 = if near > t0 { near } else { t0 };
            t1 = if far < t1 { far } else { t1 };
            if t0 > t1 {
                return (false, -1.0, -1.0);
            }
        }
        return (true, t0, t1);
    }
}
#[cfg(test)]
mod test {
    use super::Bounds3;
    use crate::extends::{Point3};
    #[test]
    pub fn get_bound() {
        let bound = Bounds3::new(Point3::new(1.0, 3.0, 5.0), Point3::new(2.0, 4.0, 6.0));
        for i in 0..8 {
            println!("{:?}", bound.rang_point(i));
        }
    }
}
