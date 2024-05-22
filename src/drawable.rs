use std::cell::RefCell;
use std::rc::Rc;

use cgmath::Matrix4;
use wgpu::{BindGroupLayout, Queue, RenderPass};

use crate::object::GameObject;
use crate::world::World;

pub(crate) trait Drawable {
    fn setup(&mut self, world: &mut World, bind_group_layout: &BindGroupLayout);
    fn update(
        &mut self,
        world: &mut World,
        parent: Rc<RefCell<GameObject>>,
        queue: &Queue,
        outer_transform: &Matrix4<f32>,
    );
    unsafe fn draw(&self, world: &World, rpass: &mut RenderPass);
}
