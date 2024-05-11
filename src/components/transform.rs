use std::cell::RefCell;
use std::rc::Rc;
use crate::components::Component;
use cgmath::{Vector3, Zero};
use crate::object::GameObject;

pub struct TransformComp {
    pub pos: Vector3<f32>,
    pub rot: Vector3<f32>,
    pub scale: Vector3<f32>,
}

impl Component for TransformComp {
    fn new() -> Self {
        TransformComp {
            pos: Vector3::zero(),
            rot: Vector3::zero(),
            scale: Vector3::zero(),
        }
    }

    fn init(&mut self) {
        
    }

    fn update(&mut self, parent: Rc<RefCell<GameObject>>) {
        
    }
}
