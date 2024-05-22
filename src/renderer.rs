use std::cell::RefCell;
use std::mem::size_of;
use std::rc::Rc;

use cgmath::{Matrix4, SquareMatrix};
use log::{debug, error};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry,
    BindingType, Buffer, BufferBindingType, BufferUsages, Color, ColorTargetState,
    ColorWrites, CommandEncoder, CommandEncoderDescriptor, CompareFunction, DepthBiasState,
    FragmentState, Id, include_wgsl, LoadOp, MultisampleState, Operations, PipelineLayout,
    RenderPass, RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor,
    RenderPipeline, ShaderStages, StencilState, StoreOp, SurfaceError, SurfaceTexture,
    TextureFormat, TextureView, TextureViewDescriptor, VertexAttribute, VertexBufferLayout,
    VertexFormat, VertexStepMode,
};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::window::Window;

use crate::asset_management::AssetManager;
use crate::asset_management::shadermanager::{Shader, ShaderId};
use crate::components::camera::CameraData;
use crate::components::CameraComp;
use crate::object::GameObject;
use crate::state::State;
use crate::world::World;

pub struct RenderContext {
    pub output: SurfaceTexture,
    pub color_view: TextureView,
    pub depth_view: TextureView,
    pub encoder: CommandEncoder,
}
pub struct Renderer {
    pub(crate) state: Box<State>,
    window: Window,
}

pub struct RuntimeRenderer {
    pub(crate) state: Box<State>,
    window: Window,
    pipeline_2d_id: ShaderId,
    pipeline_3d_id: ShaderId,
    camera_uniform_data: Box<CameraData>,
    camera_uniform_buffer: Buffer,
    camera_uniform_bind_group: BindGroup,
}

impl Renderer {
    fn create_uniform_buffer(
        camera_uniform_bind_group_layout: &BindGroupLayout,
        state: &State,
        camera_data: &CameraData,
    ) -> (Buffer, BindGroup) {
        let uniform_buffer = state.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[*camera_data]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let uniform_bind_group = state.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: &camera_uniform_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        (uniform_buffer, uniform_bind_group)
    }

    pub(crate) async fn new(window: Window) -> Self {
        let state = Box::new(State::new(&window).await);

        Renderer { window, state }
    }

    pub(crate) fn init(mut self, asset_manager: &mut AssetManager) -> RuntimeRenderer {
        let camera_data = Box::new(CameraData::empty());
        let (camera_uniform_buffer, camera_uniform_bind_group) = Self::create_uniform_buffer(
            &asset_manager
                .materials
                .shaders
                .camera_uniform_bind_group_layout,
            &self.state,
            &camera_data,
        );
        let pipeline2d = asset_manager
            .materials
            .shaders
            .load_combined_shader("2D", include_str!("shaders/shader2d.wgsl"));
        let pipeline3d = asset_manager
            .materials
            .shaders
            .load_combined_shader("3D", include_str!("shaders/shader3d.wgsl"));
        RuntimeRenderer {
            state: self.state,
            window: self.window,
            pipeline_2d_id: pipeline2d,
            pipeline_3d_id: pipeline3d,
            camera_uniform_data: camera_data,
            camera_uniform_buffer,
            camera_uniform_bind_group,
        }
    }
}

impl RuntimeRenderer {
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

        let world_ptr: *mut World = world;
        let camera_rc = unsafe { (*world_ptr).active_camera.as_ref().unwrap().upgrade() };
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

        let shader = world
            .assets
            .materials
            .shaders
            .get_shader(self.pipeline_3d_id)
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
        rpass.set_bind_group(0, &self.camera_uniform_bind_group, &[]);

        unsafe {
            self.traverse_and_render(
                &mut *world_ptr,
                &mut rpass,
                &world.children,
                Matrix4::identity(),
            );
        }
    }

    unsafe fn traverse_and_render(
        &self,
        world: &mut World,
        rpass: &mut RenderPass,
        children: &Vec<Rc<RefCell<GameObject>>>,
        combined_matrix: Matrix4<f32>,
    ) {
        let world_ptr: *mut World = world;
        for child in children {
            let child_ptr = child.as_ptr();
            if !(*child_ptr).children.is_empty() {
                self.traverse_and_render(
                    &mut *world_ptr,
                    rpass,
                    &(*child_ptr).children,
                    combined_matrix * (*child_ptr).transform.full_matrix(),
                );
            }
            let object_ptr = child.as_ptr();
            for drawable in &mut (*object_ptr).drawable {
                {
                    drawable.update(
                        &mut *world_ptr,
                        child.clone(),
                        &self.state.queue,
                        &combined_matrix,
                    );
                }
                {
                    let rpass_ptr: *mut RenderPass = rpass;
                    drawable.draw(&*world_ptr, &mut *rpass_ptr);
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
}
