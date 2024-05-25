use std::collections::HashMap;

use bytemuck::{Pod, Zeroable};
use nalgebra::Vector3;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindingResource, Buffer,
    BufferUsages, Device, Queue,
};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

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

impl Material {
    pub(crate) fn init_runtime(
        &self,
        world: &mut World,
        device: &Device,
        queue: &Queue,
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
                .get_runtime_texture_ensure_init(diffuse_texture_id, device, queue)
                .unwrap();
            let normal_texture_id = self.diffuse_texture.unwrap_or(FALLBACK_NORMAL_TEXTURE);
            // TODO: Implement normal texture into bind group
            let _normal_texture = (*world)
                .assets
                .textures
                .get_runtime_texture_ensure_init(normal_texture_id, device, queue)
                .unwrap();
            let shininess_texture_id = self.diffuse_texture.unwrap_or(FALLBACK_SHININESS_TEXTURE);
            // TODO: Implement shininess texture into bind group
            let _shininess_texture = (*world)
                .assets
                .textures
                .get_runtime_texture_ensure_init(shininess_texture_id, device, queue)
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

pub struct MaterialManager<'a> {
    materials: HashMap<usize, (Material, Option<RuntimeMaterial>)>,
    next_id: MaterialId,
    pub shaders: ShaderManager<'a>,
    device: &'a Device,
}

#[allow(dead_code)]
impl<'a> MaterialManager<'a> {
    pub fn new(device: &'a Device) -> MaterialManager<'a> {
        let shader_manager = ShaderManager::new(device);
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
            device,
        };
        manager.add_material(fallback);
        manager
    }

    pub fn add_material(&mut self, material: Material) -> MaterialId {
        let id = self.next_id;

        self.materials.insert(id, (material, None));
        self.next_id += 1;

        id
    }

    pub fn get_material_internal_mut(
        &mut self,
        id: MaterialId,
    ) -> Option<&mut (Material, Option<RuntimeMaterial>)> {
        self.materials.get_mut(&id)
    }

    pub fn get_raw_material(&self, id: MaterialId) -> Option<&Material> {
        self.materials.get(&id).map(|m| &m.0)
    }

    pub fn get_runtime_material(&self, id: MaterialId) -> Option<&RuntimeMaterial> {
        let mesh = self.materials.get(&id);
        match mesh {
            None => None,
            Some((_, opt_runtime_mesh)) => opt_runtime_mesh.as_ref(),
        }
    }

    pub fn get_runtime_material_mut(&mut self, id: MaterialId) -> Option<&mut RuntimeMaterial> {
        let mesh = self.materials.get_mut(&id);
        match mesh {
            None => None,
            Some((_, opt_runtime_mesh)) => opt_runtime_mesh.as_mut(),
        }
    }

    pub fn init_runtime_material(
        &mut self,
        world: &mut World,
        queue: &Queue,
        id: MaterialId,
        material_uniform_bind_group_layout: &BindGroupLayout,
    ) {
        self.get_runtime_material_or_init(world, queue, id, material_uniform_bind_group_layout);
    }

    pub fn get_runtime_material_or_init(
        &mut self,
        world: &mut World,
        queue: &Queue,
        id: MaterialId,
        material_uniform_bind_group_layout: &BindGroupLayout,
    ) -> Option<&RuntimeMaterial> {
        let material = self.materials.get_mut(&id);
        match material {
            None => None,
            Some((material, opt_runtime_material)) => match opt_runtime_material {
                None => {
                    let runtime_material = material.init_runtime(
                        world,
                        self.device,
                        queue,
                        material_uniform_bind_group_layout,
                    );
                    *opt_runtime_material = Some(runtime_material);
                    opt_runtime_material.as_ref()
                }
                Some(runtime_mesh) => Some(runtime_mesh),
            },
        }
    }
}
