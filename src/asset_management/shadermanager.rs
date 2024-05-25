use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::mem::size_of;
use std::path::Path;

use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
    BufferBindingType, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState,
    DepthStencilState, Device, FragmentState, MultisampleState, PipelineLayout,
    PipelineLayoutDescriptor, PrimitiveState, RenderPipeline, RenderPipelineDescriptor,
    SamplerBindingType, ShaderModule, ShaderModuleDescriptor, ShaderSource, ShaderStages,
    StencilState, TextureFormat, TextureSampleType, TextureViewDimension, VertexAttribute,
    VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
};

use crate::asset_management::mesh::Vertex3D;

pub struct Shader {
    pub name: String,
    pub module: ShaderModule,
    pub pipeline_layout: PipelineLayout,
    pub pipeline: RenderPipeline,
}

pub type ShaderId = usize;
pub const FALLBACK_SHADER_ID: ShaderId = 0;

pub struct ShaderManager<'a> {
    pub camera_uniform_bind_group_layout: BindGroupLayout,
    pub model_uniform_bind_group_layout: BindGroupLayout,
    pub material_uniform_bind_group_layout: BindGroupLayout,
    next_id: ShaderId,
    shaders: HashMap<ShaderId, Shader>,
    device: &'a Device,
}

#[allow(dead_code)]
impl<'a> ShaderManager<'a> {
    pub fn new(device: &'a Device) -> ShaderManager {
        let camera_uniform_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Camera Uniform Bind Group Layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let model_uniform_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Model Uniform Bind Group Layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let material_uniform_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Material Uniform Bind Group Layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: true },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 2,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 3,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: true },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 4,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                        count: None,
                    },
                ],
            });
        let mut shader_manager = ShaderManager {
            camera_uniform_bind_group_layout,
            model_uniform_bind_group_layout,
            material_uniform_bind_group_layout,
            next_id: 0,
            shaders: HashMap::new(),
            device,
        };
        shader_manager.load_combined_shader(
            "Fallback",
            include_str!("../shaders/fallback_shader3d.wgsl"),
        );
        shader_manager
    }

    pub fn load_combined_shader_file<T>(
        &mut self,
        name: &str,
        path: T,
    ) -> Result<ShaderId, Box<dyn Error>>
    where
        T: AsRef<Path>,
    {
        let content = fs::read_to_string(path)?;
        Ok(self.load_combined_shader(name, &content))
    }

    pub fn load_combined_shader(&mut self, name: &str, shader: &str) -> ShaderId {
        let shader = self.device.create_shader_module(ShaderModuleDescriptor {
            label: Some(name),
            source: ShaderSource::Wgsl(Cow::Borrowed(shader)),
        });
        let pipeline_layout = self
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some(&format!("{} Pipeline Layout", name)),
                bind_group_layouts: &[
                    &self.camera_uniform_bind_group_layout,
                    &self.model_uniform_bind_group_layout,
                    &self.material_uniform_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });
        let pipeline = self
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some(&format!("{} Pipeline", name)),
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: &shader,
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
                    }], // TODO: Make this cleaner
                },
                primitive: PrimitiveState::default(),
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
                    targets: &[Some(ColorTargetState {
                        format: TextureFormat::Bgra8UnormSrgb,
                        blend: None,
                        write_mask: ColorWrites::all(),
                    })],
                }),
                multiview: None,
            });

        let shader_id = self.add_shader(Shader {
            name: name.to_string(),
            module: shader,
            pipeline_layout,
            pipeline,
        });
        shader_id
    }

    pub fn add_shader(&mut self, shader: Shader) -> ShaderId {
        let id = self.next_id;

        self.shaders.insert(self.next_id, shader);
        self.next_id += 1;

        id
    }

    pub(crate) fn get_shader(&self, id: ShaderId) -> Option<&Shader> {
        self.shaders.get(&id)
    }

    pub(crate) fn find_shader_by_name(&self, name: &str) -> Option<ShaderId> {
        self.shaders
            .iter()
            .find(|(_, v)| v.name == name)
            .map(|(k, _)| k)
            .cloned()
    }
}
