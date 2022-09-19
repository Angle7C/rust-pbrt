use super::*;
use crate::extends::*;

pub struct Sphere{
    shape:BaseShape,
    pub center: Vec3,
    pub radius: f32,
    pub theta_min:f32,
    pub theta_max:f32,
    pub phi_max:f32,
    pub z_min:f32,
    pub z_max:f32,
}
impl BaseShapeAble for Sphere {
    fn area(&self) -> f32 {
        self.radius*self.radius*PI
    }
    fn intersect(&self, ray: Ray) -> Option<SurfaceInteraction> {
        todo!()
    }
    fn intersect_p(&self, ray: Ray) -> Option<SurfaceInteraction> {
        todo!()
    }
    fn new_base()->Self {
        todo!()
    }
    fn obj_to_world(&self) -> Rc<Affine3> {
        todo!()
    }
    fn object_world_bound(&self) -> Bounds {
        todo!()
    }
    fn object_bound(&self) -> Bounds {
        Bounds::new(Point3::new(-self.radius, -self.radius, self.z_min),
         Point3::new(self.radius, self.radius, self.z_max))
    }
    fn pdf(&self, interaction: &Interaction) -> f32 {
        todo!()
    }
    fn pdf_iter(&self, interaction: &Interaction, wi: &Vec3) -> f32 {
        todo!()
    }
    fn reverse_orientation(&self) -> bool {
        todo!()
    }
    fn sample(&self, u: &Point2) -> Interaction {
        todo!()
    }
    fn sample_inter(&self, interaction: &Interaction, u: &Point2) -> Interaction {
        todo!()
    }
    fn transform_swap_handedness(&self) -> bool {
        todo!()
    }
    fn world_to_world(&self) -> Rc<Affine3> {
        todo!()
    }
    
}

