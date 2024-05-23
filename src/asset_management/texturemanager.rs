use std::collections::HashMap;

use wgpu::{
    Device, Extent3d, Queue, SamplerDescriptor, TextureAspect, TextureDescriptor, TextureDimension,
    TextureFormat, TextureUsages, TextureViewDescriptor, TextureViewDimension,
};
use wgpu::util::{DeviceExt, TextureDataOrder};

pub const FALLBACK_DIFFUSE_TEXTURE: TextureId = 0;
pub const FALLBACK_NORMAL_TEXTURE: TextureId = 1;
pub const FALLBACK_SHININESS_TEXTURE: TextureId = 2;

pub struct RuntimeTexture {
    texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
    pub(crate) sampler: wgpu::Sampler,
}

pub struct RawTexture {
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub data: Vec<u8>,
}

enum Texture {
    Raw(RawTexture),
    Runtime(RuntimeTexture),
}

pub type TextureId = usize;

pub struct TextureManager<'a> {
    textures: HashMap<TextureId, Texture>,
    next_id: TextureId,
    device: &'a Device,
}

impl<'a> TextureManager<'a> {
    pub fn generate_new_fallback_diffuse_texture(
        width: u32,
        height: u32,
    ) -> Vec<u8> {
        let mut diffuse = vec![];
        for x in 0..width as i32 {
            for y in 0..height as i32 {
                if x % 2 == y % 2 {
                    diffuse.extend_from_slice(&[0, 0, 0, 255]);
                } else {
                    diffuse.extend_from_slice(&[255, 0, 255, 255]);
                }
            }
        }
        diffuse
    }

    pub fn new(device: &'a Device, queue: &Queue) -> TextureManager<'a> {
        let mut manager = TextureManager {
            textures: HashMap::new(),
            next_id: 0,
            device,
        };

        const WIDTH: u32 = 35;
        const HEIGHT: u32 = 35;
        manager.add_texture(
            WIDTH,
            HEIGHT,
            TextureFormat::Bgra8UnormSrgb,
            Self::generate_new_fallback_diffuse_texture(WIDTH, HEIGHT),
        );
        manager.add_texture(1, 1, TextureFormat::Bgra8UnormSrgb, vec![0, 0, 0, 0]);
        manager.add_texture(1, 1, TextureFormat::Bgra8UnormSrgb, vec![0, 0, 0, 0]);

        manager.get_runtime_texture_ensure_init(FALLBACK_DIFFUSE_TEXTURE, device, queue);
        manager.get_runtime_texture_ensure_init(FALLBACK_NORMAL_TEXTURE, device, queue);
        manager.get_runtime_texture_ensure_init(FALLBACK_SHININESS_TEXTURE, device, queue);

        manager
    }

    pub fn add_texture(
        &mut self,
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
        data: Vec<u8>,
    ) -> TextureId {
        let texture = Texture::Raw(RawTexture {
            width,
            height,
            format,
            data,
        });
        let id = self.next_id;

        self.textures.insert(id, texture);
        self.next_id += 1;

        id
    }

    fn get_internal_texture_mut(&mut self, texture: TextureId) -> Option<&mut Texture> {
        self.textures.get_mut(&texture)
    }

    pub fn get_raw_texture(&self, texture: TextureId) -> Option<&RawTexture> {
        let opt_tex = self.textures.get(&texture);
        match opt_tex {
            None => None,
            Some(tex) => match tex {
                Texture::Raw(tex) => Some(tex),
                Texture::Runtime(_) => None,
            },
        }
    }

    pub fn get_runtime_texture(&self, texture: TextureId) -> Option<&RuntimeTexture> {
        let opt_tex = self.textures.get(&texture);
        match opt_tex {
            None => None,
            Some(tex) => match tex {
                Texture::Raw(_) => None,
                Texture::Runtime(tex) => Some(tex),
            },
        }
    }

    pub fn get_runtime_texture_ensure_init(
        &mut self,
        texture: TextureId,
        device: &Device,
        queue: &Queue,
    ) -> Option<&RuntimeTexture> {
        let opt_tex = self.get_internal_texture_mut(texture);
        match opt_tex {
            None => None,
            Some(tex) => Some(Self::initialize_texture(tex, device, queue)),
        }
    }

    pub fn initialize_texture<'t>(
        texture: &'t mut Texture,
        device: &Device,
        queue: &Queue,
    ) -> &'t RuntimeTexture {
        match texture {
            Texture::Raw(raw) => {
                let gpu_tex = device.create_texture_with_data(
                    queue,
                    &TextureDescriptor {
                        label: Some("Texture"),
                        size: Extent3d {
                            width: raw.width,
                            height: raw.height,
                            depth_or_array_layers: 1,
                        },
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: TextureDimension::D2,
                        format: raw.format,
                        usage: TextureUsages::TEXTURE_BINDING,
                        view_formats: &[TextureFormat::Bgra8UnormSrgb],
                    },
                    TextureDataOrder::LayerMajor,
                    raw.data.as_slice(),
                );
                let view = gpu_tex.create_view(&TextureViewDescriptor {
                    label: Some("Texture View"),
                    format: Some(TextureFormat::Bgra8UnormSrgb),
                    dimension: Some(TextureViewDimension::D2),
                    aspect: TextureAspect::All,
                    base_mip_level: 0,
                    mip_level_count: None,
                    base_array_layer: 0,
                    array_layer_count: None,
                });
                let sampler = device.create_sampler(&SamplerDescriptor::default());
                let run_texture = RuntimeTexture {
                    texture: gpu_tex,
                    view,
                    sampler,
                };

                *texture = Texture::Runtime(run_texture);
                if let Texture::Runtime(run_texture) = texture {
                    run_texture
                } else {
                    unreachable!()
                }
            }
            Texture::Runtime(run) => run, // already init-ed
        }
    }
}
