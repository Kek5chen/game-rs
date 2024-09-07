use std::collections::HashMap;
use std::rc::Rc;

use bytemuck::{Pod, Zeroable};
use nalgebra::Vector3;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindingResource, Buffer,
    BufferUsages, Device, Queue,
};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::asset_management::assetmanager::DefaultGPUObjects;
use crate::asset_management::shadermanager;
use crate::asset_management::shadermanager::{ShaderId, ShaderManager};
use crate::asset_management::texturemanager::{
    FALLBACK_DIFFUSE_TEXTURE, FALLBACK_NORMAL_TEXTURE, FALLBACK_SHININESS_TEXTURE, TextureId,
};
use crate::world::World;

pub type MaterialId = usize;

pub const FALLBACK_MATERIAL_ID: usize = 0;

pub struct Material {
    pub name: String,
    pub diffuse: Vector3<f32>,
    pub diffuse_texture: Option<TextureId>,
    pub normal_texture: Option<TextureId>,
    pub shininess: f32,
    pub shininess_texture: Option<TextureId>,
    pub opacity: f32,
    pub shader: ShaderId,
}

pub struct MaterialItem {
    raw: Material,
    runtime: Option<RuntimeMaterial>,
}

impl Material {
    pub(crate) fn init_runtime(
        &self,
        world: &mut World,
        device: &Device,
        _queue: &Queue,
        material_uniform_bind_group_layout: &BindGroupLayout,
    ) -> RuntimeMaterial {
        let data = RuntimeMaterialData {
            diffuse: self.diffuse,
            _padding1: 0,
            use_diffuse_texture: self.diffuse_texture.is_some() as u32,
            use_normal_texture: self.normal_texture.is_some() as u32,
            shininess: self.shininess,
            opacity: self.opacity,
        };

        let material_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Material Data Buffer"),
            contents: bytemuck::cast_slice(&[data]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let world: *mut World = world;
        unsafe {
            let diffuse_texture_id = self.diffuse_texture.unwrap_or(FALLBACK_DIFFUSE_TEXTURE);
            let diffuse_texture = (*world)
                .assets
                .textures
                .get_runtime_texture_ensure_init(diffuse_texture_id)
                .unwrap();
            let normal_texture_id = self.diffuse_texture.unwrap_or(FALLBACK_NORMAL_TEXTURE);
            // TODO: Implement normal texture into bind group
            let normal_texture = (*world)
                .assets
                .textures
                .get_runtime_texture_ensure_init(normal_texture_id)
                .unwrap();
            let shininess_texture_id = self.diffuse_texture.unwrap_or(FALLBACK_SHININESS_TEXTURE);
            // TODO: Implement shininess texture into bind group
            let _shininess_texture = (*world)
                .assets
                .textures
                .get_runtime_texture_ensure_init(shininess_texture_id)
                .unwrap();
            let bind_group = device.create_bind_group(&BindGroupDescriptor {
                label: Some("Material Bind Group"),
                layout: material_uniform_bind_group_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: material_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::TextureView(&diffuse_texture.view),
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: BindingResource::Sampler(&diffuse_texture.sampler),
                    },
                    BindGroupEntry {
                        binding: 3,
                        resource: BindingResource::TextureView(&normal_texture.view),
                    },
                    BindGroupEntry {
                        binding: 4,
                        resource: BindingResource::Sampler(&normal_texture.sampler),
                    },
                ],
            });

            RuntimeMaterial {
                data,
                buffer: material_buffer,
                bind_group,
            }
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RuntimeMaterialData {
    diffuse: Vector3<f32>,
    _padding1: u32,
    use_diffuse_texture: u32,
    use_normal_texture: u32,
    shininess: f32,
    opacity: f32,
}

unsafe impl Zeroable for RuntimeMaterialData {}
unsafe impl Pod for RuntimeMaterialData {}

#[allow(dead_code)]
pub struct RuntimeMaterial {
    pub(crate) data: RuntimeMaterialData,
    pub(crate) buffer: Buffer,
    pub(crate) bind_group: BindGroup,
}

pub struct MaterialManager {
    materials: HashMap<usize, MaterialItem>,
    next_id: MaterialId,
    pub shaders: ShaderManager,
    device: Option<Rc<Device>>,
    queue: Option<Rc<Queue>>,
    default_gpu_objects: Option<Rc<DefaultGPUObjects>>,
}

#[allow(dead_code)]
impl MaterialManager {
    pub fn new() -> MaterialManager {
        let shader_manager = ShaderManager::new();
        let fallback = Material {
            name: "Fallback Material".to_string(),
            diffuse: Vector3::new(1.0, 1.0, 1.0),
            diffuse_texture: None,
            normal_texture: None,
            shininess: 0.0,
            shader: shadermanager::FALLBACK_SHADER_ID,
            opacity: 1.0,
            shininess_texture: None,
        };
        let mut manager = MaterialManager {
            materials: HashMap::new(),
            next_id: 0,
            shaders: shader_manager,
            device: None,
            queue: None,
            default_gpu_objects: None,
        };
        manager.add_material(fallback);
        manager
    }

    pub fn invalidate_runtime(&mut self) {
        for mat in self.materials.values_mut() {
            mat.runtime = None;
        }
        self.device = None;
        self.queue = None;
        self.default_gpu_objects = None;
        self.shaders.invalidate_runtime();
    }

    pub fn init_runtime(
        &mut self,
        device: Rc<Device>,
        queue: Rc<Queue>,
        default_gpu_objects: Rc<DefaultGPUObjects>,
    ) {
        self.device = Some(device.clone());
        self.queue = Some(queue.clone());
        self.shaders
            .init_runtime(device, default_gpu_objects.clone());
        self.default_gpu_objects = Some(default_gpu_objects)
    }

    pub fn add_material(&mut self, material: Material) -> MaterialId {
        let id = self.next_id;

        self.materials.insert(
            id,
            MaterialItem {
                raw: material,
                runtime: None,
            },
        );
        self.next_id += 1;

        id
    }

    pub fn get_material_internal_mut(&mut self, id: MaterialId) -> Option<&mut MaterialItem> {
        self.materials.get_mut(&id)
    }

    pub fn get_raw_material(&self, id: MaterialId) -> Option<&Material> {
        self.materials.get(&id).map(|m| &m.raw)
    }

    pub fn get_runtime_material(&mut self, id: MaterialId) -> Option<&RuntimeMaterial> {
        let mat = self.materials.get_mut(&id)?;
        if mat.runtime.is_none() {
            mat.runtime = Some(mat.raw.init_runtime(
                World::instance(),
                self.device.as_ref().unwrap().as_ref(),
                self.queue.as_ref().unwrap().as_ref(),
                &self.default_gpu_objects.as_ref().unwrap().material_uniform_bind_group_layout
            ));
        }
        mat.runtime.as_ref()
    }

    pub fn get_runtime_material_mut(&mut self, id: MaterialId) -> Option<&mut RuntimeMaterial> {
        let mat = self.materials.get_mut(&id)?;
        mat.runtime.as_mut()
    }

    pub fn init_runtime_material(&mut self, world: &mut World, id: MaterialId) {
        self.get_runtime_material_or_init(world, id);
    }

    pub fn get_runtime_material_or_init(
        &mut self,
        world: &mut World,
        id: MaterialId,
    ) -> Option<&RuntimeMaterial> {
        let material = self.materials.get_mut(&id)?;
        if material.runtime.is_some() {
            return material.runtime.as_ref();
        }

        let runtime_material = material.raw.init_runtime(
            world,
            self.device.as_ref().unwrap().as_ref(),
            self.queue.as_ref().unwrap().as_ref(),
            &self
                .default_gpu_objects
                .as_ref()
                .unwrap()
                .material_uniform_bind_group_layout,
        );
        material.runtime = Some(runtime_material);
        material.runtime.as_ref()
    }
}
