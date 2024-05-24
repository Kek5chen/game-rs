use std::ops::Range;

use bytemuck::{Pod, Zeroable};
use cgmath::{Vector2, Vector3};
use wgpu::{BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BufferUsages, Device};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::asset_management::materialmanager::{FALLBACK_MATERIAL_ID, MaterialId};
use crate::object::ModelData;

#[derive(Copy, Clone)]
pub struct SimpleVertex3D {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

impl SimpleVertex3D {
    pub const fn upgrade(self) -> Vertex3D {
        Vertex3D {
            position: Vector3::new(self.position[0], self.position[1], self.position[2]),
            tex_coord: Vector2::new(0.0, 0.0),
            normal: Vector3::new(self.normal[0], self.normal[1], self.normal[2]),
            tangent: Vector3::new(0.0, 0.0, 0.0),
            bitangent: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vertex3D {
    pub position: Vector3<f32>,
    pub tex_coord: Vector2<f32>,
    pub normal: Vector3<f32>,
    pub tangent: Vector3<f32>,
    pub bitangent: Vector3<f32>,
}

unsafe impl Zeroable for Vertex3D {}
unsafe impl Pod for Vertex3D {}

#[allow(dead_code)]
pub struct RuntimeMeshData {
    pub(crate) vertices_buf: wgpu::Buffer,
    pub(crate) vertices_num: usize,
    pub(crate) indices_buf: Option<wgpu::Buffer>,
    pub(crate) indices_num: usize,
    pub(crate) model_data: ModelData,
    pub(crate) model_data_buffer: wgpu::Buffer,
    pub(crate) model_bind_group: wgpu::BindGroup,
}

pub struct MeshVertexData<T> {
    pub(crate) vertices: Vec<T>,
    pub(crate) indices: Option<Vec<u32>>, // <--- put this
} //         |
  //         |
pub struct Mesh {
    //         here
    pub(crate) data: MeshVertexData<Vertex3D>,
    pub material_ranges: Vec<(MaterialId, Range<u32>)>,
}

pub struct RuntimeMesh {
    pub data: RuntimeMeshData,
}

impl Mesh {
    pub fn new(
        vertices: Vec<Vertex3D>,
        indices: Option<Vec<u32>>,
        material_ranges: Option<Vec<(MaterialId, Range<u32>)>>,
    ) -> Box<Mesh> {
        let mut material_ranges = material_ranges.unwrap_or_default();
        
        if material_ranges.is_empty() {
            if let Some(indices) = &indices {
                material_ranges.push((FALLBACK_MATERIAL_ID, 0u32..indices.len() as u32))
            } else {
                material_ranges.push((FALLBACK_MATERIAL_ID, 0u32..vertices.len() as u32))
            }
        }
        
        Box::new(Mesh {
            data: MeshVertexData::<Vertex3D> { vertices, indices },
            material_ranges,
        })
    }

    pub(crate) fn init_runtime(
        &mut self,
        device: &Device,
        model_bind_group_layout: &BindGroupLayout,
    ) -> RuntimeMesh {
        let v_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("3D Object Vertex Buffer"),
            contents: bytemuck::cast_slice(self.data.vertices.as_slice()),
            usage: BufferUsages::VERTEX,
        });
        let i_buffer = self.data.indices.as_ref().map(|indices| {
            device.create_buffer_init(&BufferInitDescriptor {
                label: Some("3D Object Index Buffer"),
                contents: bytemuck::cast_slice(indices.as_slice()),
                usage: BufferUsages::INDEX,
            })
        });
        let model_data = ModelData::empty();
        let model_data_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Model Buffer"),
            contents: bytemuck::cast_slice(&[model_data]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Model Bind Group"),
            layout: model_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: model_data_buffer.as_entire_binding(),
            }],
        });
        let runtime_mesh_data = RuntimeMeshData {
            vertices_buf: v_buffer,
            vertices_num: self.data.vertices.len(),
            indices_buf: i_buffer,
            indices_num: self
                .data
                .indices
                .as_ref()
                .map(|i| i.len())
                .unwrap_or_default(),
            model_data,
            model_data_buffer,
            model_bind_group: bind_group,
        };
        RuntimeMesh {
            data: runtime_mesh_data,
        }
    }
}
