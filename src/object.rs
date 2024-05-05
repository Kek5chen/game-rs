use bytemuck::{Pod, Zeroable};
use cgmath::{Vector2, Vector3};
use wgpu::{BindGroupLayout, BufferUsages, Device, IndexFormat, RenderPass, RenderPipeline};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

pub struct ObjectVertexData<T> {
    vertices_buf: wgpu::Buffer,
    vertices: Vec<T>,
    indices_buf: Option<wgpu::Buffer>,
    indices: Vec<u32>,
}

#[derive(Copy, Clone)]
pub struct Vertex2D {
    pub position: Vector2<f32>,
}

#[repr(C)]
pub struct Object2D {
    data: ObjectVertexData<Vertex2D>,
}

impl crate::drawable::Drawable for Object2D {
    fn draw<'a>(
        &'a self,
        rpass: &mut RenderPass<'a>,
        pipeline: &RenderPipeline,
        bind_group: &Vec<BindGroupLayout>,
    ) {
        rpass.set_vertex_buffer(0, self.data.vertices_buf.slice(..));
        if let Some(indices) = self.data.indices_buf.as_ref() {
            rpass.set_index_buffer(indices.slice(..), IndexFormat::Uint32);
        } else {
            rpass.draw(0..self.data.vertices.len() as u32, 0..1)
        }
    }
}

#[derive(Copy, Clone)]
pub struct Vertex3D {
    pub position: [f32; 3],
    pub color: [f32; 3], 
    pub normal: [f32; 3],
}

unsafe impl Zeroable for Vertex3D {}
unsafe impl Pod for Vertex3D {}

#[repr(C)]
pub struct Object3D {
    data: ObjectVertexData<Vertex3D>,
}

impl crate::drawable::Drawable for Object3D {
    fn draw<'a>(
        &'a self,
        rpass: &mut RenderPass<'a>,
        pipeline: &RenderPipeline,
        bind_group: &Vec<BindGroupLayout>,
    ) {
        rpass.set_vertex_buffer(0, self.data.vertices_buf.slice(..));
        if let Some(indices) = self.data.indices_buf.as_ref() {
            rpass.set_index_buffer(indices.slice(..), IndexFormat::Uint32);
            rpass.draw_indexed(0..self.data.indices.len() as u32, 0, 0..1);
        } else {
            rpass.draw(0..self.data.vertices.len() as u32, 0..1)
        }
    }
}

impl Object3D {
    pub fn new(device: &Device, vertices: Vec<Vertex3D>, indices: Option<Vec<u32>>) -> Object3D {
        let v_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("3D Object Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices.as_slice()),
            usage: BufferUsages::VERTEX,
        });
        let i_buffer = indices.as_ref().map(|vec| device.create_buffer_init(&BufferInitDescriptor {
                label: Some("3D Object Index Buffer"),
                contents: bytemuck::cast_slice(vec.as_slice()),
                usage: BufferUsages::INDEX,
            }));
        Object3D {
            data: ObjectVertexData {
                vertices_buf: v_buffer,
                vertices,
                indices_buf: i_buffer,
                indices: indices.unwrap_or_default(),
            },
        }
    }
}
