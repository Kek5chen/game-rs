use std::cell::RefCell;
use std::rc::Rc;

use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, Vector2, Vector3};
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BufferUsages, Device, IndexFormat, Queue,
    RenderPass,
};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::object::{GameObject, ModelData};

pub(crate) trait Drawable {
    fn setup(&mut self, device: &Device, bind_group_layout: &BindGroupLayout);
    fn update(
        &mut self,
        parent: Rc<RefCell<GameObject>>,
        queue: &Queue,
        outer_transform: &Matrix4<f32>,
    );
    fn draw<'a, 'b>(&'a self, rpass: &mut RenderPass<'b>)
    where
        'a: 'b;
}

pub struct ObjectRuntimeData {
    vertices_buf: wgpu::Buffer,
    indices_buf: Option<wgpu::Buffer>,
    model_data: ModelData,
    model_data_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

pub struct ObjectVertexData<T> {
    vertices: Vec<T>,
    indices: Option<Vec<u32>>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vertex2D {
    pub position: Vector2<f32>,
}

unsafe impl Zeroable for Vertex2D {}
unsafe impl Pod for Vertex2D {}

#[repr(C)]
pub struct Object2D {
    data: ObjectVertexData<Vertex2D>,
    runtime_data: Option<ObjectRuntimeData>,
}

impl Drawable for Object2D {
    fn setup(&mut self, device: &Device, model_bind_group_layout: &BindGroupLayout) {
        todo!()
    }

    fn update(
        &mut self,
        parent: Rc<RefCell<GameObject>>,
        queue: &Queue,
        outer_transform: &Matrix4<f32>,
    ) {
        todo!()
    }

    fn draw<'a, 'b>(&'a self, rpass: &mut RenderPass<'b>)
    where
        'a: 'b,
    {
        todo!()
    }
}

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

#[repr(C)]
pub struct Object3D {
    data: ObjectVertexData<Vertex3D>,
    runtime_data: Option<ObjectRuntimeData>,
}

impl Drawable for Object3D {
    fn setup(&mut self, device: &Device, bind_group_layout: &BindGroupLayout) {
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
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: model_data_buffer.as_entire_binding(),
            }],
        });
        self.runtime_data = Some(ObjectRuntimeData {
            vertices_buf: v_buffer,
            indices_buf: i_buffer,
            model_data,
            model_data_buffer,
            bind_group,
        });
    }

    fn update(
        &mut self,
        parent: Rc<RefCell<GameObject>>,
        queue: &Queue,
        outer_transform: &Matrix4<f32>,
    ) {
        let runtime_data = self
            .runtime_data
            .as_mut()
            .expect("Runtime data should have been setup before calling update on an object.");
        runtime_data
            .model_data
            .update(parent.clone(), outer_transform);
        queue.write_buffer(
            &runtime_data.model_data_buffer,
            0,
            bytemuck::cast_slice(&[runtime_data.model_data]),
        )
    }

    fn draw<'a, 'b>(&'a self, rpass: &mut RenderPass<'b>)
    where
        'a: 'b,
    {
        let runtime_data = self
            .runtime_data
            .as_ref()
            .expect("Runtime data should have been setup before calling draw on an object.");
        rpass.set_bind_group(1, &runtime_data.bind_group, &[]);
        rpass.set_vertex_buffer(0, runtime_data.vertices_buf.slice(..));
        if let (Some(i_buffer), Some(indices)) = (
            runtime_data.indices_buf.as_ref(),
            self.data.indices.as_ref(),
        ) {
            rpass.set_index_buffer(i_buffer.slice(..), IndexFormat::Uint32);
            rpass.draw_indexed(0..indices.len() as u32, 0, 0..1);
        } else {
            rpass.draw(0..self.data.vertices.len() as u32, 0..1)
        }
    }
}

impl Object3D {
    pub fn new(vertices: Vec<Vertex3D>, indices: Option<Vec<u32>>) -> Box<Self> {
        Box::new(Object3D {
            data: ObjectVertexData { vertices, indices },
            runtime_data: None,
        })
    }
}
