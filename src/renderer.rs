use wgpu::{CommandEncoder, CommandEncoderDescriptor, SurfaceError, SurfaceTexture, TextureView, TextureViewDescriptor};
use winit::window::Window;
use crate::state::State;

pub struct RenderContext {
    pub output: SurfaceTexture,
    pub color_view: TextureView,
    pub depth_view: TextureView,
    pub encoder: CommandEncoder,
}
pub struct Renderer {
    pub(crate) state: State,
    window: Window,
}

impl Renderer {
    pub(crate) async fn new(window: Window) -> Renderer {
        Renderer {
            state: State::new(&window).await,
            window
        }
    }
    
    pub fn begin_render(&mut self) -> Result<RenderContext, SurfaceError>
    {
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

        Ok(RenderContext { output, color_view, depth_view, encoder })
    }
    
    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn end_render(&mut self, ctx: RenderContext) {
        self.state.queue.submit(Some(ctx.encoder.finish()));
        ctx.output.present();
        self.window.request_redraw();
    }
}
