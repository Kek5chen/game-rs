use wgpu::{Device, Queue};

use crate::asset_management::{MaterialManager, TextureManager};
use crate::asset_management::meshmanager::MeshManager;

pub struct AssetManager<'a> {
    pub textures: TextureManager<'a>,
    pub materials: MaterialManager<'a>,
    pub meshes: MeshManager<'a>,
}

impl<'a> AssetManager<'a> {
    pub fn new(device: &'a Device, queue: &Queue) -> AssetManager<'a> {
        AssetManager {
            textures: TextureManager::new(device, queue),
            materials: MaterialManager::new(device),
            meshes: MeshManager::new(device),
        }
    }
}
