use crate::drawable::Drawable;
use crate::object::{Object2D, Object3D};
use crate::state::State;
use wgpu::{
    BindGroupLayout, Color, CommandEncoder, CommandEncoderDescriptor, Id, LoadOp, Operations,
    RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor,
    RenderPipeline, ShaderModule, StoreOp, SurfaceError, SurfaceTexture, TextureView,
    TextureViewDescriptor,
};
use winit::window::Window;

pub struct RenderContext {
    pub output: SurfaceTexture,
    pub color_view: TextureView,
    pub depth_view: TextureView,
    pub encoder: CommandEncoder,
}
pub struct Renderer {
    pub(crate) state: State,
    window: Window,
    pipelines: Vec<(RenderPipeline, Vec<BindGroupLayout>)>,
    shaders: Vec<ShaderModule>,
    pipeline_2d_id: Option<Id<RenderPipeline>>,
    objects_2d: Vec<Object2D>,
    pipeline_3d_id: Option<Id<RenderPipeline>>,
    objects_3d: Vec<Object3D>,
}

impl Renderer {
    pub(crate) async fn new(window: Window) -> Renderer {
        Renderer {
            state: State::new(&window).await,
            window,
            pipelines: vec![],
            shaders: vec![],
            pipeline_2d_id: None,
            objects_2d: vec![],
            pipeline_3d_id: None,
            objects_3d: vec![],
        }
    }

    pub fn begin_render(&mut self) -> Result<RenderContext, SurfaceError> {
        let output = self.state.surface.get_current_texture()?;
        let color_view = output
            .texture
            .create_view(&TextureViewDescriptor::default());
        // let depth_view = self.depth_texture.create_view(&TextureViewDescriptor {
        //     label: Some("Depth Texture View"),
        //     format: Some(TextureFormat::Depth32Float),
        //     dimension: Some(TextureViewDimension::D2),
        //     aspect: TextureAspect::DepthOnly,
        //     base_mip_level: 0,
        //     mip_level_count: None,
        //     base_array_layer: 0,
        //     array_layer_count: None,
        // });
        let depth_view = self
            .state
            .depth_texture
            .create_view(&TextureViewDescriptor::default());
        let encoder = self
            .state
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());

        Ok(RenderContext {
            output,
            color_view,
            depth_view,
            encoder,
        })
    }

    pub(crate) fn render(&mut self, ctx: &mut RenderContext) {
        let mut rpass = ctx.encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &ctx.color_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &ctx.depth_view,
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0f32),
                    store: StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        if let Some(pipeline_id) = self.pipeline_3d_id {
            let (pipeline, bind_group_layout) = self.find_pipeline(pipeline_id).unwrap();
            rpass.set_pipeline(pipeline);
            for o2d in &self.objects_2d {
                o2d.draw(&mut rpass, pipeline, bind_group_layout);
            }
        }

        if let Some(pipeline_id) = self.pipeline_3d_id {
            let (pipeline, bind_group_layout) = self.find_pipeline(pipeline_id).unwrap();
            rpass.set_pipeline(pipeline);
            for o3d in &self.objects_3d {
                o3d.draw(&mut rpass, pipeline, bind_group_layout);
            }
        }
    }

    pub fn end_render(&mut self, ctx: RenderContext) {
        self.state.queue.submit(Some(ctx.encoder.finish()));
        ctx.output.present();
        self.window.request_redraw();
    }

    pub fn find_pipeline(
        &self,
        id: Id<RenderPipeline>,
    ) -> Option<&(RenderPipeline, Vec<BindGroupLayout>)> {
        self.pipelines.iter().find(|p| id == p.0.global_id())
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
