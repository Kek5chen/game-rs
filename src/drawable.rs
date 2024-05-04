use wgpu::{BindGroupLayout, CommandEncoder, RenderPass, RenderPipeline};

pub(crate) trait Drawable {
    fn draw<'a>(
        &'a self,
        rpass: &mut RenderPass<'a>,
        pipeline: &RenderPipeline,
        bind_group: &Vec<BindGroupLayout>,
    );
}
