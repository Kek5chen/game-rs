use std::cell::RefCell;
use std::rc::Rc;
use crate::components::Component;
use cgmath::{Matrix4, SquareMatrix, Vector3, Zero};
use crate::object::GameObject;

#[repr(C)]
pub struct TransformComp {
    pub pos: Vector3<f32>,
    pub rot: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub pos_mat: Matrix4<f32>,
    pub rot_mat: Matrix4<f32>,
    pub scale_mat: Matrix4<f32>,
    pub combined_mat: Matrix4<f32>,
}

impl Component for TransformComp {
    fn new() -> Self {
        TransformComp {
            pos: Vector3::zero(),
            rot: Vector3::zero(),
            scale: Vector3::zero(),
            pos_mat: Matrix4::identity(),
            rot_mat: Matrix4::identity(),
            scale_mat: Matrix4::identity(),
            combined_mat: Matrix4::identity(),
        }
    }

    fn init(&mut self) {
        
    }

    fn update(&mut self, parent: Rc<RefCell<GameObject>>) {
        
    }
}
