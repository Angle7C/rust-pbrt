use std::rc::Rc;

use super::{shape::BaseShapeAble, medium::Medium};

pub struct Scene{
    //光源
    // light :Box<Vec<>>
    //图元
    shapes:Vec<Box<dyn BaseShapeAble>>,
    //场景介质
    medium:Option<Rc<Medium>>
}