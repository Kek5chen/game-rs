use std::cell::RefCell;
use std::rc::Rc;
use cgmath::{Matrix4, SquareMatrix, Vector3, Zero};
use crate::components::Component;
use crate::object::GameObject;

pub struct CameraComp {
    pub projection: Matrix4<f32>,
}

impl Component for CameraComp {
    fn new() -> Self {
        CameraComp {
            projection: Matrix4::identity(),
        }
    }

    fn init(&mut self) {

    }

    fn update(&mut self, parent: Rc<RefCell<GameObject>>) {
    }
}
