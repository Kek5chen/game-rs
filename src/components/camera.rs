use crate::components::Component;
use crate::object::GameObject;
use crate::transform::Transform;
use bytemuck::{Pod, Zeroable};
use cgmath::{Deg, Matrix4, SquareMatrix, Vector3, Zero};
use std::cell::RefCell;
use std::rc::Rc;

pub struct CameraComp {
    pub projection: Matrix4<f32>,
}

impl CameraComp {
    pub fn resize(&mut self, width: f32, height: f32) {
        self.projection = cgmath::perspective(Deg(60.0), width / height, 0.01, 1000.0);
    }
}

impl Component for CameraComp {
    fn new() -> Self {
        CameraComp {
            projection: cgmath::perspective(Deg(60.0), 800.0 / 600.0, 0.01, 1000.0),
        }
    }

    fn init(&mut self, parent: &mut GameObject) {
        parent.transform.set_invert_position(true);
    }

    fn update(&mut self, _parent: Rc<RefCell<GameObject>>, _delta_time: f32) {}
}

// TODO: Remove manual padding somehow?
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CameraData {
    pos: Vector3<f32>,
    _padding0: f32,
    rot: Vector3<f32>,
    _padding1: f32,
    scale: Vector3<f32>,
    _padding2: f32,
    view_mat: Matrix4<f32>,
    projection_mat: Matrix4<f32>,
    pub proj_view_mat: Matrix4<f32>,
}

impl CameraData {
    pub fn empty() -> Self {
        CameraData {
            pos: Vector3::zero(),
            _padding0: 0.0,
            rot: Vector3::zero(),
            _padding1: 0.0,
            scale: Vector3::zero(),
            _padding2: 0.0,
            view_mat: Matrix4::identity(),
            projection_mat: Matrix4::identity(),
            proj_view_mat: Matrix4::identity(),
        }
    }
    pub fn update(&mut self, proj_matrix: &Matrix4<f32>, cam_transform: &Transform) {
        self.pos = *cam_transform.position();
        self.rot = *cam_transform.rotation();
        self.scale = *cam_transform.scale();
        self.view_mat = *cam_transform.full_matrix();
        self.projection_mat = *proj_matrix;
        self.proj_view_mat = proj_matrix * cam_transform.full_matrix();
    }
}

unsafe impl Zeroable for CameraData {}
unsafe impl Pod for CameraData {}
