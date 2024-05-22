use std::collections::HashMap;

use wgpu::Device;

pub struct Texture {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

type TextureId = usize;

pub struct TextureManager<'a> {
    textures: HashMap<TextureId, Texture>,
    next_id: TextureId,
    device: &'a Device,
}

impl<'a> TextureManager<'a> {
    pub fn new(device: &'a Device) -> TextureManager {
        TextureManager {
            textures: HashMap::new(),
            next_id: 0,
            device,
        }
    }
    fn load_texture() {}
}
