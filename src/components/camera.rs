use std::cell::RefCell;
use std::rc::Rc;

use bytemuck::{Pod, Zeroable};
use nalgebra::{Matrix4, Perspective3, Vector3};
use num_traits::Zero;

use crate::components::Component;
use crate::object::GameObject;
use crate::transform::Transform;

pub struct CameraComp {
    pub projection: Perspective3<f32>,
}

impl CameraComp {
    pub fn resize(&mut self, width: f32, height: f32) {
        self.projection = Perspective3::new(width / height, 60f32.to_radians(), 0.01, 1000.0);
    }
}

impl Component for CameraComp {
    fn new() -> Self {
        CameraComp {
            projection: Perspective3::new(800.0 / 600.0, 60f32.to_radians(), 0.01, 1000.0),
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
    pub fn update(&mut self, proj_matrix: &Perspective3<f32>, cam_transform: &Transform) {
        self.pos = *cam_transform.position();
        self.rot = *cam_transform.rotation();
        self.scale = *cam_transform.scale();
        self.view_mat = *cam_transform.full_matrix();
        self.projection_mat = proj_matrix.to_homogeneous();
        self.proj_view_mat = self.projection_mat * cam_transform.full_matrix();
    }
}

unsafe impl Zeroable for CameraData {}
unsafe impl Pod for CameraData {}
