use std::rc::Rc;

use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
    BufferBindingType, Device, Queue, SamplerBindingType, ShaderStages, TextureSampleType,
    TextureViewDimension,
};

use crate::asset_management::{MaterialManager, TextureManager};
use crate::asset_management::meshmanager::MeshManager;
use crate::asset_management::shadermanager::ShaderManager;

pub struct DefaultGPUObjects {
    pub camera_uniform_bind_group_layout: BindGroupLayout,
    pub material_uniform_bind_group_layout: BindGroupLayout,
    pub model_uniform_bind_group_layout: BindGroupLayout,
}

pub struct AssetManager {
    pub textures: TextureManager,
    pub shaders: ShaderManager,
    pub materials: MaterialManager,
    pub meshes: MeshManager,
    pub default_gpu_objects: Option<Rc<DefaultGPUObjects>>,
}

impl AssetManager {
    pub fn new() -> AssetManager {
        AssetManager {
            textures: TextureManager::new(),
            shaders: ShaderManager::new(),
            materials: MaterialManager::new(),
            meshes: MeshManager::new(),
            default_gpu_objects: None,
        }
    }

    pub fn invalidate(&mut self) {
        self.textures.invalidate_runtime();
        self.shaders.invalidate_runtime();
        self.materials.invalidate_runtime();
        self.meshes.invalidate_runtime();
    }

    fn init(&mut self, device: Rc<Device>) {
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
        self.default_gpu_objects = Some(Rc::new(DefaultGPUObjects {
            camera_uniform_bind_group_layout,
            model_uniform_bind_group_layout,
            material_uniform_bind_group_layout,
        }))
    }

    pub fn init_runtime(&mut self, device: Rc<Device>, queue: Rc<Queue>) {
        self.init(device.clone());
        let gpu_objs = self.default_gpu_objects.clone().unwrap();
        
        self.textures.init_runtime(device.clone(), queue.clone(), gpu_objs.clone());
        self.shaders.init_runtime(device.clone(), gpu_objs.clone());
        self.materials.init_runtime(device.clone(), queue.clone(), gpu_objs.clone());
        self.meshes.init_runtime(device.clone(), gpu_objs.clone());
    }
}
