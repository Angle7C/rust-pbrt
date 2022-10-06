// std

// see sphere.h

use std::f64::consts::PI;

use cgmath::{Deg, EuclideanSpace, InnerSpace, MetricSpace, Rad, Zero};

use crate::{
    core::{
        aabb::Bounds3,
        interaction::{Interaction, SurfaceInteraction},
        material::Material,
        ray::Ray,
    },
    extends::{Mat4, Point2, Point3, Vector3},
    until::{transform::Transforms, untils::quadratic},
};

#[derive(Clone, Debug)]
pub struct Sphere {
    pub radius: f64,
    pub z_min: f64,
    pub z_max: f64,
    pub theta_min: f64,
    pub theta_max: f64,
    pub phi_max: f64,
    // inherited from class Shape (see shape.h)
    pub object_to_world: Transforms,
    pub reverse_orientation: bool,
    // pub transform_swaps_handedness: bool,
    pub material: Option<Material>,
}

impl Default for Sphere {
    fn default() -> Self {
        let object_to_world: Transforms = Transforms::default();
        Sphere {
            // Shape
            object_to_world,
            reverse_orientation: false,
            // transform_swaps_handedness: object_to_world.swaps_handedness(),
            // Sphere
            radius: 1.0,
            z_min: -1.0,
            z_max: 1.0,
            theta_min: (-1.0 as f64).acos(),
            theta_max: (1.0 as f64).acos(),
            phi_max: Rad(360.0).0,
            material: None,
        }
    }
}

impl Sphere {
    pub fn new(
        object_to_world: Mat4,
        reverse_orientation: bool,
        radius: f64,
        z_min: f64,
        z_max: f64,
        phi_max: f64,
    ) -> Self {
        Sphere {
            // Shape
            object_to_world: Transforms::new(object_to_world),
            reverse_orientation,
            // transform_swaps_handedness: object_to_world
            // Sphere
            radius,
            z_min: f64::clamp(z_min.min(z_max), -radius, radius),
            z_max: f64::clamp(z_min.max(z_max), -radius, radius),
            theta_min: f64::clamp(z_min.min(z_max) / radius, -1.0, 1.0).acos(),
            theta_max: f64::clamp(z_min.max(z_max) / radius, -1.0, 1.0).acos(),
            phi_max: Rad::from(Deg(f64::clamp(phi_max, 0.0, 360.0))).0,
            material: None,
        }
    }
    // Shape
    pub fn object_bound(&self) -> Bounds3 {
        Bounds3 {
            min: Point3 {
                x: -self.radius,
                y: -self.radius,
                z: self.z_min,
            },
            max: Point3 {
                x: self.radius,
                y: self.radius,
                z: self.z_max,
            },
        }
    }
    pub fn world_bound(&self) -> Bounds3 {
        // in C++: Bounds3f Shape::WorldBound() const { return (*ObjectToWorld)(ObjectBound()); }
        self.object_to_world.applying_box_3(&self.object_bound())
    }
    pub fn intersect(&self, ray: &Ray, isect: &mut SurfaceInteraction) -> bool {
        let ray: Ray = self.object_to_world.applying_ray_inv(ray);
        let ox = ray.o.x;
        let oy = ray.o.y;
        let oz = ray.o.z;
        let dx = ray.d.x;
        let dy = ray.d.y;
        let dz = ray.d.z;
        let a = dx * dx + dy * dy + dz * dz;
        let b = (dx * ox + dy * oy + dz * oz) * 2.0;
        let c = ox * ox + oy * oy + oz * oz - self.radius * self.radius;
        let t0: f64;
        let t1: f64;
        if let Some((x1, x2)) = quadratic(a, b, c) {
            t0 = x1;
            t1 = x2;
        } else {
            return false;
        }
        if t0 > ray.t_max || t1 <= 0.0 {
            return false;
        }
        let mut t_shape_hit = t0;
        if t_shape_hit <= 0.0 {
            t_shape_hit = t1;
            if t_shape_hit > ray.t_max {
                return false;
            }
        }
        let mut p_hit: Point3 = ray.at(t_shape_hit);
        p_hit *= self.radius / p_hit.distance(Point3::origin());
        if p_hit.x == 0.0 && p_hit.y == 0.0 {
            p_hit.x = 1e-5 * self.radius;
        }
        let mut phi: f64 = p_hit.y.atan2(p_hit.x);
        if phi < 0.0 {
            phi += 2.0 * PI;
        }
        if (self.z_min > -self.radius && p_hit.z < self.z_min)
            || (self.z_max < self.radius && p_hit.z > self.z_max)
            || phi > self.phi_max
        {
            if t_shape_hit == t1 {
                return false;
            }
            if t1 > ray.t_max as f64 {
                return false;
            }
            t_shape_hit = t1;
            p_hit = ray.at(t_shape_hit);

            p_hit *= self.radius / p_hit.distance(Point3::origin());
            if p_hit.x == 0.0 && p_hit.y == 0.0 {
                p_hit.x = 1e-5 * self.radius;
            }
            phi = p_hit.y.atan2(p_hit.x);
            if phi < 0.0 {
                phi += 2.0 * PI;
            }
            if (self.z_min > -self.radius && p_hit.z < self.z_min)
                || (self.z_max < self.radius && p_hit.z > self.z_max)
                || phi > self.phi_max
            {
                return false;
            }
        }
        let u: f64 = phi / self.phi_max;
        let theta: f64 = f64::clamp(p_hit.z / self.radius, -1.0, 1.0).acos();
        let v: f64 = (theta - self.theta_min) / (self.theta_max - self.theta_min);
        let z_radius: f64 = (p_hit.x * p_hit.x + p_hit.y * p_hit.y).sqrt();
        let inv_z_radius: f64 = 1.0 / z_radius;
        let cos_phi: f64 = p_hit.x * inv_z_radius;
        let sin_phi: f64 = p_hit.y * inv_z_radius;
        let dpdu: Vector3 = Vector3 {
            x: -self.phi_max * p_hit.y,
            y: self.phi_max * p_hit.x,
            z: 0.0,
        };
        let dpdv: Vector3 = Vector3 {
            x: p_hit.z * cos_phi,
            y: p_hit.z * sin_phi,
            z: -self.radius * theta.sin(),
        } * (self.theta_max - self.theta_min);
        let d2_p_duu: Vector3 = Vector3 {
            x: p_hit.x,
            y: p_hit.y,
            z: 0.0,
        } * -self.phi_max
            * self.phi_max;
        let d2_p_duv: Vector3 = Vector3 {
            x: -sin_phi,
            y: cos_phi,
            z: 0.0,
        } * (self.theta_max - self.theta_min)
            * p_hit.z
            * self.phi_max;
        let d2_p_dvv: Vector3 = Vector3 {
            x: p_hit.x,
            y: p_hit.y,
            z: p_hit.z,
        } * -(self.theta_max - self.theta_min)
            * (self.theta_max - self.theta_min);
        let ec: f64 = dpdu.dot(dpdu);
        let fc: f64 = dpdu.dot(dpdv);
        let gc: f64 = dpdv.dot(dpdv);
        let nc: Vector3 = dpdu.cross(dpdv).normalize();
        let el: f64 = nc.dot(d2_p_duu);
        let fl: f64 = nc.dot(d2_p_duv);
        let gl: f64 = nc.dot(d2_p_dvv);
        let inv_egf2: f64 = 1.0 / (ec * gc - fc * fc);
        let dndu = dpdu * (fl * fc - el * gc) * inv_egf2 + dpdv * (el * fc - fl * ec) * inv_egf2;
        let dndv = dpdu * (gl * fc - fl * gc) * inv_egf2 + dpdv * (fl * fc - gl * ec) * inv_egf2;
        let _uv_hit: Point2 = Point2 { x: u, y: v };
        let w0: Vector3 = -ray.d;
        *isect = SurfaceInteraction::new(
            self.object_to_world.applying_point(p_hit),
            t_shape_hit,
            self.object_to_world.applying_vector(w0),
            self.object_to_world.applying_normal((p_hit - Point3::origin()).normalize()),
        );
        true
    }
    pub fn intersect_p(&self, ray: &Ray) -> bool {
        let ray: Ray = self.object_to_world.applying_ray_inv(ray);
        let ox = ray.o.x;
        let oy = ray.o.y;
        let oz = ray.o.z;
        let dx = ray.d.x;
        let dy = ray.d.y;
        let dz = ray.d.z;
        let a = dx * dx + dy * dy + dz * dz;
        let b = (dx * ox + dy * oy + dz * oz) * 2.0;
        let c = ox * ox + oy * oy + oz * oz - self.radius * self.radius;
        let t0: f64;
        let t1: f64;
        if let Some((x1, x2)) = quadratic(a, b, c) {
            t0 = x1;
            t1 = x2;
        } else {
            return false;
        }
        if t0 > ray.t_max || t1 <= 0.0 {
            return false;
        }
        let mut t_shape_hit = t0;
        if t_shape_hit <= 0.0 {
            t_shape_hit = t1;
            if t_shape_hit > ray.t_max {
                return false;
            }
        }

        let mut p_hit: Point3 = ray.at(t_shape_hit);
        p_hit *= self.radius / p_hit.distance(Point3::origin());
        if p_hit.x == 0.0 && p_hit.y == 0.0 {
            p_hit.x = 1e-5 * self.radius;
        }
        let mut phi: f64 = p_hit.y.atan2(p_hit.x);
        if phi < 0.0 {
            phi += 2.0 * PI;
        }
        if (self.z_min > -self.radius && p_hit.z < self.z_min)
            || (self.z_max < self.radius && p_hit.z > self.z_max)
            || phi > self.phi_max
        {
            if t_shape_hit == t1 {
                return false;
            }
            if t1 > ray.t_max as f64 {
                return false;
            }
            t_shape_hit = t1;
            p_hit = ray.at(t_shape_hit);
            p_hit *= self.radius / p_hit.distance(Point3::origin());
            if p_hit.x == 0.0 && p_hit.y == 0.0 {
                p_hit.x = 1e-5 * self.radius;
            }
            phi = p_hit.y.atan2(p_hit.x);
            if phi < 0.0 {
                phi += 2.0 * PI;
            }
            if (self.z_min > -self.radius && p_hit.z < self.z_min)
                || (self.z_max < self.radius && p_hit.z > self.z_max)
                || phi > self.phi_max
            {
                return false;
            }
        }
        let _t = Interaction::new(
            self.get_object_to_world().applying_point(p_hit),
            t_shape_hit,
            -ray.d,
            self.get_object_to_world()
                .applying_vector(p_hit - Point3::origin()),
            None,
            None,
        );
        true
    }
    pub fn get_reverse_orientation(&self) -> bool {
        self.reverse_orientation
    }
    pub fn get_transform_swaps_handedness(&self) -> bool {
        todo!()
    }
    pub fn get_object_to_world(&self) -> Transforms {
        self.object_to_world
    }
    pub fn area(&self) -> f64 {
        self.phi_max * self.radius * (self.z_max - self.z_min)
    }
    pub fn sample(&self, _u: Point2, _pdf: &mut f64) -> Interaction {
        todo!()
    }
    pub fn sample_with_ref_point(
        &self,
        _iref: &Interaction,
        _u: Point2,
        _pdf: &mut f64,
    ) -> Interaction {
        todo!()
    }
    pub fn pdf_with_ref_point(&self, _iref: &Interaction, _wi: &Vector3) -> f64 {
        todo!()
    }
}
