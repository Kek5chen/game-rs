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
    pub position: Vector3<f32>,
}

unsafe impl Zeroable for Vertex3D {}
unsafe impl Pod for Vertex3D {}

#[repr(C)]
pub struct Object3D {
    data: ObjectVertexData<Vertex3D>,
}

impl crate::drawable::Drawable for Object3D {
    fn draw(
        &self,
        rpass: &mut RenderPass,
        pipeline: &RenderPipeline,
        bind_group: &Vec<BindGroupLayout>,
    ) {
        todo!()
    }
}
