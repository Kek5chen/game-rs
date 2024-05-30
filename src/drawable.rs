use std::cell::RefCell;
use std::rc::Rc;

use nalgebra::Matrix4;
use wgpu::{BindGroupLayout, Device, Queue, RenderPass};

use crate::object::GameObject;
use crate::world::World;

pub(crate) trait Drawable {
    fn setup(
        &mut self,
        device: &Device,
        queue: &Queue,
        world: &mut World,
        model_bind_group_layout: &BindGroupLayout,
        material_bind_group_layout: &BindGroupLayout,
    );
    fn update(
        &mut self,
        world: &mut World,
        parent: Rc<RefCell<Box<GameObject>>>,
        queue: &Queue,
        outer_transform: &Matrix4<f32>,
    );
    unsafe fn draw(&self, world: &World, rpass: &mut RenderPass);
}
