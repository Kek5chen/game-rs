use wgpu::{BindGroupLayout, Device, RenderPass, RenderPipeline};

pub(crate) trait Drawable {
    fn setup(&mut self, device: &Device);
    fn draw<'a>(
        &'a self,
        rpass: &mut RenderPass<'a>,
        pipeline: &RenderPipeline,
        bind_group: &Vec<BindGroupLayout>,
    );
}
