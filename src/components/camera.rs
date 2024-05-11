use std::cell::RefCell;
use std::rc::Rc;
use cgmath::{Matrix4, SquareMatrix, Vector3, Zero};
use crate::components::Component;
use crate::object::GameObject;

pub struct CameraComp {
    projection: Matrix4<f32>,
    view: Matrix4<f32>,
    model: Matrix4<f32>,
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: Vector3<f32>,
}

impl Component for CameraComp {
    fn new() -> Self {
        CameraComp {
            projection: Matrix4::identity(),
            view: Matrix4::identity(),
            model: Matrix4::identity(),
            position: Vector3::zero(),
            rotation: Vector3::zero(),
            scale: Vector3::zero(),
        }
    }

    fn init(&mut self) {

    }

    fn update(&mut self, parent: Rc<RefCell<GameObject>>) {
    }
}
