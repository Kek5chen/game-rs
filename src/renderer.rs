use crate::buffer::{CUBE, CUBE_INDICES};
use crate::drawable::Drawable;
use crate::object::{GameObject, Object2D, Object3D, Vertex2D, Vertex3D};
use crate::state::State;
use crate::world::World;
use cgmath::Vector3;
use log::error;
use std::cell::RefMut;
use wgpu::{
    include_wgsl, BindGroupLayout, Color, ColorTargetState, ColorWrites, CommandEncoder,
    CommandEncoderDescriptor, CompareFunction, DepthBiasState, FragmentState, Id, LoadOp,
    MultisampleState, Operations, PipelineLayout, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, RenderPassDescriptor, RenderPipeline, StencilState, StoreOp,
    SurfaceError, SurfaceTexture, TextureFormat, TextureView, TextureViewDescriptor,
    VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode,
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
    pipeline_2d_id: Id<RenderPipeline>,
    pipeline_3d_id: Id<RenderPipeline>,
}

impl Renderer {
    fn make_2d_pipeline(state: &State) -> (PipelineLayout, RenderPipeline) {
        let pipeline_2d_layout =
            state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("2D Render Pipeline"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        let shader_2d = state
            .device
            .create_shader_module(include_wgsl!("shaders/shader2d.wgsl"));

        let pipeline_2d = state
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("2D Render Pipeline"),
                layout: Some(&pipeline_2d_layout),
                vertex: wgpu::VertexState {
                    module: &shader_2d,
                    entry_point: "vs_main",
                    buffers: &[VertexBufferLayout {
                        step_mode: VertexStepMode::Vertex,
                        attributes: &[VertexAttribute {
                            format: VertexFormat::Float32x2,
                            offset: 0,
                            shader_location: 0,
                        }],
                        array_stride: std::mem::size_of::<Vertex2D>() as u64,
                    }],
                },
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: CompareFunction::Less,
                    stencil: StencilState::default(),
                    bias: DepthBiasState::default(),
                }),
                multisample: MultisampleState::default(),
                fragment: Some(FragmentState {
                    module: &shader_2d,
                    entry_point: "fs_main",
                    targets: &[Some(ColorTargetState {
                        blend: None,
                        format: TextureFormat::Bgra8UnormSrgb,
                        write_mask: ColorWrites::all(),
                    })],
                }),
                multiview: None,
            });

        (pipeline_2d_layout, pipeline_2d)
    }

    fn make_3d_pipeline(state: &State) -> (PipelineLayout, RenderPipeline) {
        let pipeline_3d_layout =
            state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("3D Render Pipeline"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        let shader_3d = state
            .device
            .create_shader_module(include_wgsl!("shaders/shader3d.wgsl"));

        let pipeline_3d = state
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("3D Render Pipeline"),
                layout: Some(&pipeline_3d_layout),
                vertex: wgpu::VertexState {
                    module: &shader_3d,
                    entry_point: "vs_main",
                    buffers: &[VertexBufferLayout {
                        step_mode: VertexStepMode::Vertex,
                        attributes: &[
                            VertexAttribute {
                                format: VertexFormat::Float32x3,
                                offset: 0,
                                shader_location: 0,
                            },
                            VertexAttribute {
                                format: VertexFormat::Float32x3,
                                offset: std::mem::size_of::<Vector3<f32>>() as u64,
                                shader_location: 1,
                            },
                            VertexAttribute {
                                format: VertexFormat::Float32x3,
                                offset: std::mem::size_of::<Vector3<f32>>() as u64 * 2,
                                shader_location: 2,
                            },
                        ],
                        array_stride: std::mem::size_of::<Vertex3D>() as u64,
                    }],
                },
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: CompareFunction::Less,
                    stencil: StencilState::default(),
                    bias: DepthBiasState::default(),
                }),
                multisample: MultisampleState::default(),
                fragment: Some(FragmentState {
                    module: &shader_3d,
                    entry_point: "fs_main",
                    targets: &[Some(ColorTargetState {
                        blend: None,
                        format: TextureFormat::Bgra8UnormSrgb,
                        write_mask: ColorWrites::all(),
                    })],
                }),
                multiview: None,
            });

        (pipeline_3d_layout, pipeline_3d)
    }
    pub(crate) async fn new(window: Window) -> Renderer {
        let state = State::new(&window).await;

        let (pipeline_2d_layout, pipeline_2d) = Self::make_2d_pipeline(&state);
        let (pipeline_3d_layout, pipeline_3d) = Self::make_3d_pipeline(&state);
        Renderer {
            window,
            pipeline_2d_id: pipeline_2d.global_id(),
            pipeline_3d_id: pipeline_3d.global_id(),
            pipelines: vec![(pipeline_2d, vec![]), (pipeline_3d, vec![])],
            state,
        }
    }

    pub fn render_world(&mut self, world: &mut World) -> bool {
        let ctx = match self.begin_render() {
            Ok(ctx) => Some(ctx),
            Err(SurfaceError::Lost) => {
                self.state.resize(self.state.size);
                None
            }
            Err(SurfaceError::OutOfMemory) => {
                error!("The application ran out of memory");
                None
            }
            Err(e) => {
                error!("{:?}", e);
                None
            }
        };

        if ctx.is_none() {
            return false;
        }

        let mut ctx = ctx.unwrap();

        self.render(&mut ctx, world);
        self.end_render(ctx);

        true
    }

    fn begin_render(&mut self) -> Result<RenderContext, SurfaceError> {
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

    fn render(&mut self, ctx: &mut RenderContext, world: &mut World) {
        let (pipeline, bind_group_layout) = self.find_pipeline(self.pipeline_3d_id).unwrap();
        let obj_rcs: Vec<RefMut<GameObject>> =
            world.objects.iter().map(|obj| obj.borrow_mut()).collect();
        let drawable_rcs = obj_rcs
            .iter()
            .filter_map(|o| o.drawable.as_ref())
            .collect::<Vec<&Box<dyn Drawable>>>();

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

        rpass.set_pipeline(pipeline);
        for drawable in drawable_rcs {
            drawable.draw(&mut rpass, pipeline, bind_group_layout);
        }
    }

    fn end_render(&mut self, ctx: RenderContext) {
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
