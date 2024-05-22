use std::collections::HashMap;

use cgmath::Vector3;
use wgpu::core::id::TextureId;
use wgpu::Device;

use crate::asset_management::shadermanager;
use crate::asset_management::shadermanager::{ShaderId, ShaderManager};

type MaterialId = usize;

const FALLBACK_MATERIAL_ID: usize = 0;

enum Diffuse {
    Color(Vector3<u8>),
    Texture(TextureId),
}

pub struct Material {
    diffuse: Diffuse,
    shader: ShaderId,
}

pub struct MaterialManager<'a> {
    materials: HashMap<usize, Material>,
    next_id: MaterialId,
    fallback_material: MaterialId,
    pub shaders: ShaderManager<'a>,
}

impl<'a> MaterialManager<'a> {
    pub fn new(device: &'a Device) -> MaterialManager<'a> {
        let shader_manager = ShaderManager::new(device);
        let fallback = Material {
            diffuse: Diffuse::Color(Vector3::new(255, 255, 255)),
            shader: shadermanager::FALLBACK_SHADER_ID,
        };
        let mut manager = MaterialManager {
            materials: HashMap::new(),
            next_id: 0,
            fallback_material: FALLBACK_MATERIAL_ID,
            shaders: shader_manager,
        };
        manager.add_material(fallback);
        manager
    }

    pub fn add_material(&mut self, material: Material) -> MaterialId {
        let id = self.next_id;

        self.materials.insert(id, material);
        self.next_id += 1;

        id
    }

    fn load_texture() {}
}
