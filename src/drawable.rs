use wgpu::{BindGroupLayout, Device, RenderPass, RenderPipeline};

pub(crate) trait Drawable {
    fn setup(&mut self, device: &Device);
    fn draw<'a, 'b>(
        &'a self,
        rpass: &mut RenderPass<'b>,
        pipeline: &RenderPipeline,
        bind_group: &BindGroupLayout,
    ) where
        'a: 'b;
}
