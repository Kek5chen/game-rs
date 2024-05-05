use cgmath::{Vector3, Zero};
use crate::components::Component;

pub struct TransformComp {
    pub pos: Vector3<f32>,
    pub rot: Vector3<f32>,
    pub scale: Vector3<f32>,
}

impl Component for TransformComp {}

impl Default for TransformComp {
    fn default() -> Self {
        TransformComp {
            pos: Vector3::zero(),
            rot: Vector3::zero(),
            scale: Vector3::zero()
        }
    }
}
