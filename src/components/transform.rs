use std::cell::RefCell;
use std::rc::Rc;
use crate::components::Component;
use cgmath::{Matrix4, SquareMatrix, Vector3, Zero, Deg};
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

impl TransformComp {
    fn update_matrices(&mut self) {
        self.pos_mat = Matrix4::from_translation(self.pos);
        self.rot_mat = Matrix4::from_angle_x(Deg(self.rot.x)) * Matrix4::from_angle_y(Deg(self.rot.y)) * Matrix4::from_angle_z(Deg(self.rot.z));
        self.scale_mat = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        self.combined_mat = self.scale_mat * self.rot_mat * self.pos_mat;
    }
}

impl Component for TransformComp {
    fn new() -> Self {
        TransformComp {
            pos: Vector3::zero(),
            rot: Vector3::zero(),
            scale: Vector3::new(1.0,1.0,1.0),
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
