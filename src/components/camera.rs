use std::cell::RefCell;
use std::rc::Rc;
use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, SquareMatrix, Vector3, Zero};
use crate::components::{Component, TransformComp};
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
    proj_view_mat: Matrix4<f32>,
}

impl CameraData {
    pub fn new(proj_matrix: &Matrix4<f32>, cam_transform: &TransformComp) -> Self {
        CameraData {
            pos: cam_transform.pos,
            _padding0: 0f32,
            rot: cam_transform.rot,
            _padding1: 0f32,
            scale: cam_transform.scale,
            _padding2: 0f32,
            view_mat: cam_transform.combined_mat,
            projection_mat: *proj_matrix,
            proj_view_mat: proj_matrix * cam_transform.combined_mat
        }
    }
}

unsafe impl Zeroable for CameraData {}
unsafe impl Pod for CameraData {}
