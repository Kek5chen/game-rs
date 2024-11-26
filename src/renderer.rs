use std::cell::RefCell;
use std::rc::Rc;

use log::{debug, error};
use nalgebra::{Matrix4, Perspective3};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::*;
use winit::window::Window;

use crate::asset_management::assetmanager::DefaultGPUObjects;
use crate::asset_management::shadermanager::{ShaderId, DIM3_SHADER_ID, FALLBACK_SHADER_ID};
use crate::components::camera::CameraData;
use crate::components::CameraComp;
use crate::object::GameObjectId;
use crate::state::State;
use crate::world::World;

pub struct RenderContext {
    pub output: SurfaceTexture,
    pub color_view: TextureView,
    pub depth_view: TextureView,
    pub encoder: CommandEncoder,
}

pub struct CameraRenderData {
    camera_uniform_data: Box<CameraData>,
    camera_uniform_buffer: Buffer,
    camera_uniform_bind_group: BindGroup,
}

#[allow(dead_code)]
pub struct Renderer {
    pub(crate) state: Box<State>,
    window: Window,
    current_pipeline: Option<ShaderId>,
    camera_render_data: Option<CameraRenderData>,
    default_gpu_objects: Option<Rc<DefaultGPUObjects>>,
}

impl Renderer {
    pub fn create_uniform_init(
        bind_group_layout: &BindGroupLayout,
        state: &State,
        data: &'_ [u8],
    ) -> (Buffer, BindGroup) {
        let uniform_buffer = state.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: data,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let uniform_bind_group = state.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        (uniform_buffer, uniform_bind_group)
    }

    pub fn create_uniform_buffer(
        bind_group_layout: &BindGroupLayout,
        state: &State,
        data_size: usize,
    ) -> (Buffer, BindGroup) {
        let uniform_buffer = state.device.create_buffer(&BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: data_size as BufferAddress,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: true,
        });

        let uniform_bind_group = state.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        (uniform_buffer, uniform_bind_group)
    }

    pub(crate) async fn new(window: Window) -> Self {
        let state = Box::new(State::new(&window).await);

        Renderer {
            state,
            window,
            current_pipeline: None,
            camera_render_data: None,
            default_gpu_objects: None,
        }
    }

    pub fn init(&mut self, default_gpu_objects: Rc<DefaultGPUObjects>) {
        // TODO: Make it possible to pick a shader
        self.current_pipeline = Some(DIM3_SHADER_ID);
        self.default_gpu_objects = Some(default_gpu_objects);
        let camera_data = Box::new(CameraData::empty());
        let (camera_uniform_buffer, camera_uniform_bind_group) = Self::create_uniform_init(
            &self
                .default_gpu_objects
                .as_ref()
                .unwrap()
                .camera_uniform_bind_group_layout,
            &self.state,
            bytemuck::cast_slice(&[*camera_data]),
        );
        self.camera_render_data = Some(CameraRenderData {
            camera_uniform_data: camera_data,
            camera_uniform_buffer,
            camera_uniform_bind_group,
        });
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
        let mut output = self.state.surface.get_current_texture()?;
        if output.suboptimal {
            drop(output);
            self.state.recreate_surface();
            output = self.state.surface.get_current_texture()?;
        }

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
        if self.current_pipeline.is_none() {
            self.current_pipeline = Some(FALLBACK_SHADER_ID);
        }

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

        if self.current_pipeline.is_none() {
            debug!("No pipeline active");
            return;
        }

        let current_pipeline = self.current_pipeline.unwrap();

        let render_data = self
            .camera_render_data
            .as_mut()
            .expect("Camera render data should be initialized");

        let world_ptr: *mut World = world;
        let camera_rc = unsafe { (*world_ptr).active_camera.as_ref().unwrap() };

        let camera = camera_rc;
        let camera_comp: Option<Rc<RefCell<Box<CameraComp>>>> =
            camera.get_component::<CameraComp>();
        if camera_comp.is_none() {
            debug!("Camera didn't have a camera component");
            return;
        }

        let camera_comp = camera_comp.unwrap();
        let projection_matrix: &Perspective3<f32> = &camera_comp.borrow_mut().projection;
        let camera_transform = &camera.transform;
        render_data
            .camera_uniform_data
            .update(projection_matrix, camera_transform);
        self.state.queue.write_buffer(
            &render_data.camera_uniform_buffer,
            0,
            bytemuck::cast_slice(&[*render_data.camera_uniform_data]),
        );

        let shader = world
            .assets
            .shaders
            .get_shader(current_pipeline)
            .expect("3D Pipeline should've been initialized previously");

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

        rpass.set_pipeline(&shader.pipeline);
        rpass.set_bind_group(0, &render_data.camera_uniform_bind_group, &[]);

        unsafe {
            self.traverse_and_render(
                &mut *world_ptr,
                &mut rpass,
                &mut world.children,
                Matrix4::identity(),
            );
        }
    }

    unsafe fn traverse_and_render(
        &self,
        world: &mut World,
        rpass: &mut RenderPass,
        children: &mut [GameObjectId],
        combined_matrix: Matrix4<f32>,
    ) {
        let world_ptr: *mut World = world;
        for child in children {
            if !child.children.is_empty() {
                self.traverse_and_render(
                    &mut *world_ptr,
                    rpass,
                    &mut child.clone().children,
                    combined_matrix * child.transform.full_matrix().to_homogeneous(),
                );
            }
            if let Some(drawable) = &mut child.clone().drawable {
                {
                    drawable.update(&mut *world_ptr, *child, &self.state.queue, &combined_matrix);
                }
                {
                    let rpass_ptr: *mut RenderPass = rpass;
                    drawable.draw(&mut *world_ptr, &mut *rpass_ptr);
                }
            }
        }
    }

    fn end_render(&mut self, ctx: RenderContext) {
        self.state.queue.submit(Some(ctx.encoder.finish()));
        ctx.output.present();
        self.window.request_redraw();
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }
}
