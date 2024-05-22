use wgpu::Device;
use crate::asset_management::{MaterialManager, TextureManager};

pub struct AssetManager<'a> {
    pub textures: TextureManager<'a>,
    pub materials: MaterialManager<'a>,
}

impl<'a> AssetManager<'a> {
    pub fn new(device: &'a Device) -> AssetManager {
        AssetManager {
            textures: TextureManager::new(device),
            materials: MaterialManager::new(device),
        }
    }
}
