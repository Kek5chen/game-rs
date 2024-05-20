use std::cell::RefCell;
use std::rc::Rc;

use wgpu::{BindGroupLayout, Device, Queue, RenderPass, RenderPipeline};

use crate::object::GameObject;

pub(crate) trait Drawable {
    fn setup(&mut self, device: &Device, bind_group_layout: &BindGroupLayout);
    fn update(&mut self, parent: Rc<RefCell<GameObject>>, queue: &Queue);
    fn draw<'a, 'b>(
        &'a self,
        rpass: &mut RenderPass<'b>,
        pipeline: &RenderPipeline,
        bind_group: &BindGroupLayout,
    ) where
        'a: 'b;
}
