mod app;
mod buffer;
mod renderer;
mod state;

use crate::app::App;
use env_logger::Env;
use log::LevelFilter;
use std::error::Error;
use wgpu::{Color, LoadOp, Operations, RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor, StoreOp};
use crate::renderer::RenderContext;

fn render(ctx: &mut RenderContext) {
    ctx.encoder.begin_render_pass(&RenderPassDescriptor {
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let log_env = Env::new().filter("RUST_LOG");
    env_logger::builder()
        .parse_default_env() // Default env
        .filter_level(LevelFilter::Info)// Use at least info level
        .parse_env(log_env) // Or override with whatever env says
        .init();

    App::default().run(render).await;

    Ok(())
}
