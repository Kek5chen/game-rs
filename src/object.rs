use wgpu::{BindGroupLayout, IndexFormat, RenderPass, RenderPipeline};

pub struct ObjectVertexData {
    vertices: wgpu::Buffer,
    vertices_len: u32,
    indices: Option<wgpu::Buffer>,
    indices_len: u32,
}

pub struct Object2D {
    data: ObjectVertexData,
}

impl crate::drawable::Drawable for Object2D {
    fn draw<'a>(
        &'a self,
        rpass: &mut RenderPass<'a>,
        pipeline: &RenderPipeline,
        bind_group: &Vec<BindGroupLayout>,
    ) {
        rpass.set_vertex_buffer(0, self.data.vertices.slice(..));
        if let Some(indices) = self.data.indices.as_ref() {
            rpass.set_index_buffer(indices.slice(..), IndexFormat::Uint32);
        } else {
            rpass.draw(0..self.data.vertices_len, 0..1)
        }
    }
}

pub struct Object3D {
    data: ObjectVertexData,
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
