use std::rc::Rc;
use wgpu::{Device, Queue};

use crate::asset_management::{MaterialManager, TextureManager};
use crate::asset_management::meshmanager::MeshManager;

pub struct AssetManager {
    pub textures: TextureManager,
    pub materials: MaterialManager,
    pub meshes: MeshManager,
}

impl AssetManager {
    pub fn new(device: Rc<Device>, queue: Rc<Queue>) -> AssetManager {
        AssetManager {
            textures: TextureManager::new(device.clone(), queue),
            materials: MaterialManager::new(device.clone()),
            meshes: MeshManager::new(device),
        }
    }
}
