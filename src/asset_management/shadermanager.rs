use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::rc::Rc;
use wgpu::*;

use crate::asset_management::assetmanager::DefaultGPUObjects;
use crate::asset_management::mesh::Vertex3D;

pub struct ShaderItem {
    raw: Shader,
    runtime: Option<RuntimeShader>,
}

pub struct Shader {
    pub name: String,
    pub code: String,
}

pub struct RuntimeShader {
    pub name: String,
    pub module: ShaderModule,
    pub pipeline_layout: PipelineLayout,
    pub pipeline: RenderPipeline,
}

pub type ShaderId = usize;
// The fallback shader if a pipeline fails
pub const FALLBACK_SHADER_ID: ShaderId = 0;
// The default 3D shader.
pub const DIM3_SHADER_ID: ShaderId = 1;

pub struct ShaderManager {
    next_id: ShaderId,
    shaders: HashMap<ShaderId, ShaderItem>,
    device: Option<Rc<Device>>,
    default_gpu_objects: Option<Rc<DefaultGPUObjects>>,
}

impl Shader {
    pub fn initialize_combined_runtime(
        &mut self,
        device: &Device,
        camera_uniform_bind_group_layout: &BindGroupLayout,
        model_uniform_bind_group_layout: &BindGroupLayout,
        material_uniform_bind_group_layout: &BindGroupLayout,
    ) -> RuntimeShader {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some(&self.name),
            source: ShaderSource::Wgsl(Cow::Borrowed(&self.code)),
        });
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some(&format!("{} Pipeline Layout", self.name)),
            bind_group_layouts: &[
                camera_uniform_bind_group_layout,
                model_uniform_bind_group_layout,
                material_uniform_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some(&format!("{} Pipeline", self.name)),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[Vertex3D::continuous_descriptor()],
            },
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState::default(),
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(ColorTargetState {
                    format: TextureFormat::Bgra8UnormSrgb,
                    blend: None,
                    write_mask: ColorWrites::all(),
                })],
            }),
            multiview: None,
            cache: None,
        });
        RuntimeShader {
            name: self.name.clone(),
            module: shader,
            pipeline_layout,
            pipeline,
        }
    }
}

#[allow(dead_code)]
impl ShaderManager {
    pub fn init(&mut self) {}

    pub fn new() -> ShaderManager {
        let mut shader_manager = ShaderManager {
            next_id: 0,
            shaders: HashMap::new(),
            device: None,
            default_gpu_objects: None,
        };
        shader_manager.add_shader(
            "Fallback".to_string(),
            include_str!("../shaders/fallback_shader3d.wgsl").to_string(),
        );
        shader_manager.add_shader(
            "3D Default Pipeline".to_string(),
            include_str!("../shaders/shader3d.wgsl").to_string(),
        );
        shader_manager
    }

    pub fn invalidate_runtime(&mut self) {
        for shader in self.shaders.values_mut() {
            shader.runtime = None;
        }
        self.device = None;
        self.default_gpu_objects = None;
    }

    pub fn init_runtime(&mut self, device: Rc<Device>, default_gpu_objects: Rc<DefaultGPUObjects>) {
        self.device = Some(device);
        self.default_gpu_objects = Some(default_gpu_objects);
        self.init();
    }

    pub fn add_combined_shader_file<T>(
        &mut self,
        name: &str,
        path: T,
    ) -> Result<ShaderId, Box<dyn Error>>
    where
        T: AsRef<Path>,
    {
        let content = fs::read_to_string(path)?;
        Ok(self.add_combined_shader(name, &content))
    }

    pub fn add_combined_shader(&mut self, name: &str, shader: &str) -> ShaderId {
        self.add_shader(name.to_string(), shader.to_string())
    }

    pub fn add_shader(&mut self, name: String, code: String) -> ShaderId {
        let id = self.next_id;

        self.shaders.insert(
            self.next_id,
            ShaderItem {
                raw: Shader { name, code },
                runtime: None,
            },
        );
        self.next_id += 1;

        id
    }

    pub(crate) fn get_shader(&mut self, id: ShaderId) -> Option<&RuntimeShader> {
        let shader_item = self.shaders.get_mut(&id)?;
        if shader_item.runtime.is_none() {
            let default_gpu_objects = self.default_gpu_objects.as_ref().unwrap().as_ref();
            let runtime_shader = shader_item.raw.initialize_combined_runtime(
                self.device.clone().unwrap().as_ref(),
                &default_gpu_objects.camera_uniform_bind_group_layout,
                &default_gpu_objects.model_uniform_bind_group_layout,
                &default_gpu_objects.material_uniform_bind_group_layout,
            );
            shader_item.runtime = Some(runtime_shader);
        }
        shader_item.runtime.as_ref()
    }

    pub(crate) fn find_shader_by_name(&self, name: &str) -> Option<ShaderId> {
        self.shaders
            .iter()
            .find(|(_, v)| v.raw.name == name)
            .map(|(k, _)| k)
            .cloned()
    }
}
