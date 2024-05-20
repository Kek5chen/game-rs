use std::cell::RefCell;
use std::mem::size_of;
use std::rc::Rc;

use cgmath::{Matrix4, SquareMatrix};
use log::{debug, error};
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, Color, ColorTargetState, ColorWrites, CommandEncoder, CommandEncoderDescriptor, CompareFunction, DepthBiasState, FragmentState, Id, include_wgsl, LoadOp, MultisampleState, Operations, PipelineLayout, RenderPass, RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor, RenderPipeline, ShaderStages, StencilState, StoreOp, SurfaceError, SurfaceTexture, TextureFormat, TextureView, TextureViewDescriptor, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::window::Window;

use crate::components::camera::CameraData;
use crate::components::CameraComp;
use crate::object::{GameObject, Vertex2D, Vertex3D};
use crate::state::State;
use crate::world::World;

pub struct RenderContext {
    pub output: SurfaceTexture,
    pub color_view: TextureView,
    pub depth_view: TextureView,
    pub encoder: CommandEncoder,
}
pub struct Renderer {
    pub(crate) state: State,
    window: Window,
    pipelines: Vec<RenderPipeline>,
    pipeline_2d_id: Id<RenderPipeline>,
    pipeline_3d_id: Id<RenderPipeline>,
    uniform_bind_group_layout: BindGroupLayout,
    camera_uniform_data: Box<CameraData>,
    camera_uniform_buffer: Buffer,
    pub(crate) model_bind_group_layout: BindGroupLayout,
    uniform_bind_group: BindGroup,
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

    fn make_3d_pipeline(
        state: &State,
        uniform_bind_group_layout: &BindGroupLayout,
        model_bind_group_layout: &BindGroupLayout,
    ) -> (PipelineLayout, RenderPipeline) {
        let pipeline_3d_layout =
            state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("3D Render Pipeline"),
                    bind_group_layouts: &[uniform_bind_group_layout, model_bind_group_layout],
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
                                format: VertexFormat::Float32x2,
                                offset: 3 * 4, // one vec3
                                shader_location: 1,
                            },
                            VertexAttribute {
                                format: VertexFormat::Float32x3,
                                offset: 3 * 4 + 2 * 4, // one vec3 and a vec2
                                shader_location: 2,
                            },
                            VertexAttribute {
                                format: VertexFormat::Float32x3,
                                offset: 2 * 4 * 3 + 2 * 4, // two vec3 and a vec2
                                shader_location: 3,
                            },
                            VertexAttribute {
                                format: VertexFormat::Float32x3,
                                offset: 3 * 4 * 3 + 2 * 4, // three vec3 and a vec2
                                shader_location: 4,
                            },
                        ],
                        array_stride: size_of::<Vertex3D>() as u64,
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

    fn create_uniform_buffer(
        state: &State,
        camera_data: &CameraData,
    ) -> (BindGroupLayout, Buffer, BindGroup) {
        let uniform_bind_group_layout =
            state
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("Uniform Bind Group Layout"),
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::VERTEX,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });

        let uniform_buffer = state.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[*camera_data]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let uniform_bind_group = state.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        (
            uniform_bind_group_layout,
            uniform_buffer,
            uniform_bind_group,
        )
    }

    pub(crate) async fn new(window: Window) -> Renderer {
        let state = State::new(&window).await;

        let camera_data = Box::new(CameraData::empty());
        let (uniform_bind_group_layout, camera_uniform_buffer, uniform_bind_group) =
            Self::create_uniform_buffer(&state, &camera_data);
        let model_bind_group_layout =
            state
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("Model Bind Group Layout"),
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::VERTEX,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });
        let (pipeline_2d_layout, pipeline_2d) = Self::make_2d_pipeline(&state);
        let (pipeline_3d_layout, pipeline_3d) =
            Self::make_3d_pipeline(&state, &uniform_bind_group_layout, &model_bind_group_layout);
        Renderer {
            window,
            pipeline_2d_id: pipeline_2d.global_id(),
            pipeline_3d_id: pipeline_3d.global_id(),
            pipelines: vec![pipeline_2d, pipeline_3d],
            uniform_bind_group_layout,
            camera_uniform_data: camera_data,
            camera_uniform_buffer,
            uniform_bind_group,
            model_bind_group_layout,
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
        if world.active_camera.is_none() {
            debug!("No camera active");
            return;
        }

        let camera_rc = world.active_camera.as_ref().unwrap().upgrade();
        if camera_rc.is_none() {
            debug!("Couldn't take ownership of camera");
            return;
        }

        let camera = camera_rc.unwrap();
        let camera_comp: Option<Rc<RefCell<Box<CameraComp>>>> =
            camera.borrow_mut().get_component::<CameraComp>();
        if camera_comp.is_none() {
            debug!("Camera didn't have a camera component");
            return;
        }

        let camera_comp = camera_comp.unwrap();
        let projection_matrix: &Matrix4<f32> = &camera_comp.borrow_mut().projection;
        let camera_transform = &camera.borrow().transform;
        self.camera_uniform_data
            .update(projection_matrix, camera_transform);
        self.state.queue.write_buffer(
            &self.camera_uniform_buffer,
            0,
            bytemuck::cast_slice(&[*self.camera_uniform_data]),
        );

        let pipeline = self.find_pipeline(self.pipeline_3d_id).unwrap();

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
        rpass.set_bind_group(0, &self.uniform_bind_group, &[]);

        unsafe {
            self.traverse_and_render(&mut rpass, &pipeline, &world.children, Matrix4::identity());
        }
    }
    
    unsafe fn traverse_and_render(&self, rpass: &mut RenderPass, pipeline: &RenderPipeline, children: &Vec<Rc<RefCell<GameObject>>>, combined_matrix: Matrix4<f32>) {
        for child in children {
           let child_ptr = child.as_ptr();
            if !(*child_ptr).children.is_empty() {
                self.traverse_and_render(rpass, pipeline, &(*child_ptr).children, combined_matrix * (*child_ptr).transform.full_matrix());
            }
            let object_ptr = child.as_ptr();
            for drawable in &mut (*object_ptr).drawable {
                drawable.update(child.clone(), &self.state.queue, &combined_matrix);
                drawable.draw(rpass, pipeline, &self.uniform_bind_group_layout);
            }
        }
    }

    fn end_render(&mut self, ctx: RenderContext) {
        self.state.queue.submit(Some(ctx.encoder.finish()));
        ctx.output.present();
        self.window.request_redraw();
    }

    pub fn find_pipeline(&self, id: Id<RenderPipeline>) -> Option<&RenderPipeline> {
        self.pipelines.iter().find(|p| id == p.global_id())
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
