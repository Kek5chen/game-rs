use std::any::Any;

use nalgebra::Matrix4;
use wgpu::{BindGroupLayout, Device, Queue, RenderPass};

use crate::object::GameObjectId;
use crate::world::World;

pub(crate) trait Drawable: Any {
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
        parent: GameObjectId,
        queue: &Queue,
        outer_transform: &Matrix4<f32>,
    );
    unsafe fn draw(&self, world: &World, rpass: &mut RenderPass);
}
