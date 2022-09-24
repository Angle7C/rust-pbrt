use crate::extends::{Point2, Point3, Vec3, Vec2};
pub struct Bounds3 {
    pub min: Point3,
    pub max: Point3,
}
impl Bounds3 {
    pub fn init() -> Self {
        Self {
            min: (Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY)),
            max: (Point3::new(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY)),
        }
    }
    pub fn init_point(point: Point3) -> Self {
        Self {
            min: (point),
            max: (point),
        }
    }
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min: min, max: max }
    }
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
    pub fn inside(&self, p: &Point3) -> bool {
        return p.x >= self.min.x
            && p.x <= self.max.x
            && p.y >= self.min.y
            && p.y <= self.max.y
            && p.z >= self.min.z
            && p.z <= self.max.z;
    }
    pub fn expend(&self, det: Point3) -> Self {
        Self::new(self.min - det, self.max + det)
    }
    pub fn diagonal(&self) -> Vec3 {
        self.max - self.min
    }
    pub fn lerp(&self) -> Self {
        todo!()
    }
    pub fn offset(&self, _p: &Point3) -> Vec3 {
        todo!()
    }
    pub fn bound_sphere(&self) -> Self {
        todo!()
    }
    #[allow(unused_comparisons)]
    pub fn rang_point(&self, i: usize) -> Vec3 {
        if i > 8 || i < 0 {
            panic!("the index is out")
        }
        let x = if i & 1 == 0 { self.min.x } else { self.max.x };
        let y = if i & 2 == 0 { self.min.y } else { self.max.y };
        let z = if i & 4 == 0 { self.min.z } else { self.max.z };
        Vec3::new(x, y, z)
    }
    pub fn area(&self)->f32{
        let det=self.max-self.min;
        det.x.abs()*det.y.abs()
    }
}

pub struct Bounds2 {
    pub min: Point2,
    pub max: Point2,
}
impl Bounds2 {
    pub fn area(&self)->f32{
        let det=self.max-self.min;
        det.x.abs()*det.y.abs()
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
        return p.x >= self.min.x
            && p.x <= self.max.x
            && p.y >= self.min.y
            && p.y <= self.max.y
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
