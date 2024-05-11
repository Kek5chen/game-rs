use crate::components::{Component, TransformComp};
use crate::drawable::Drawable;
use bytemuck::{Pod, Zeroable};
use cgmath::Vector2;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::rc::Rc;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{BindGroupLayout, BufferUsages, Device, IndexFormat, RenderPass, RenderPipeline};

pub struct ObjectRuntimeData {
    vertices_buf: wgpu::Buffer,
    indices_buf: Option<wgpu::Buffer>,
}

pub struct ObjectVertexData<T> {
    vertices: Vec<T>,
    indices: Vec<u32>,
}

#[derive(Copy, Clone)]
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
    fn setup(&mut self, device: &Device) {
        let v_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("3D Object Vertex Buffer"),
            contents: bytemuck::cast_slice(self.data.vertices.as_slice()),
            usage: BufferUsages::VERTEX,
        });
        let i_buffer = if self.data.indices.is_empty() {
            None
        } else {
            Some(device.create_buffer_init(&BufferInitDescriptor {
                label: Some("3D Object Index Buffer"),
                contents: bytemuck::cast_slice(self.data.indices.as_slice()),
                usage: BufferUsages::INDEX,
            }))
        };
        self.runtime_data = Some(ObjectRuntimeData {
            vertices_buf: v_buffer,
            indices_buf: i_buffer,
        });
    }

    fn draw<'a, 'b>(
        &'a self,
        rpass: &mut RenderPass<'b>,
        pipeline: &RenderPipeline,
        bind_group: &Vec<BindGroupLayout>,
    ) where
        'a: 'b,
    {
        let runtime_data = self
            .runtime_data
            .as_ref()
            .expect("Runtime data should have been setup before calling draw on an object.");
        rpass.set_vertex_buffer(0, runtime_data.vertices_buf.slice(..));
        if let Some(indices) = runtime_data.indices_buf.as_ref() {
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
    runtime_data: Option<ObjectRuntimeData>,
}

impl Drawable for Object3D {
    fn setup(&mut self, device: &Device) {
        let v_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("3D Object Vertex Buffer"),
            contents: bytemuck::cast_slice(self.data.vertices.as_slice()),
            usage: BufferUsages::VERTEX,
        });
        let i_buffer = if self.data.indices.is_empty() {
            None
        } else {
            Some(device.create_buffer_init(&BufferInitDescriptor {
                label: Some("3D Object Index Buffer"),
                contents: bytemuck::cast_slice(self.data.indices.as_slice()),
                usage: BufferUsages::INDEX,
            }))
        };

        self.runtime_data = Some(ObjectRuntimeData {
            vertices_buf: v_buffer,
            indices_buf: i_buffer,
        });
    }

    fn draw<'a, 'b>(
        &'a self,
        rpass: &mut RenderPass<'b>,
        pipeline: &RenderPipeline,
        bind_group: &Vec<BindGroupLayout>,
    ) where
        'a: 'b,
    {
        let runtime_data = self
            .runtime_data
            .as_ref()
            .expect("Runtime data should have been setup before calling draw on an object.");
        rpass.set_vertex_buffer(0, runtime_data.vertices_buf.slice(..));
        if let Some(indices) = runtime_data.indices_buf.as_ref() {
            rpass.set_index_buffer(indices.slice(..), IndexFormat::Uint32);
            rpass.draw_indexed(0..self.data.indices.len() as u32, 0, 0..1);
        } else {
            rpass.draw(0..self.data.vertices.len() as u32, 0..1)
        }
    }
}

impl Object3D {
    pub fn new(vertices: Vec<Vertex3D>, indices: Option<Vec<u32>>) -> Object3D {
        Object3D {
            data: ObjectVertexData {
                vertices,
                indices: indices.unwrap_or_default(),
            },
            runtime_data: None,
        }
    }
}

pub struct GameObject {
    pub name: String,
    pub children: Vec<Rc<RefCell<GameObject>>>,
    pub transform: TransformComp,
    pub drawable: Option<Box<dyn Drawable>>,
    pub components: Vec<Rc<RefCell<Box<dyn Component>>>>,
}

impl GameObject {
    pub fn add_child(&mut self, child: Rc<RefCell<GameObject>>) {
        // TODO: Make the children know who it's owned by because of circling references
        self.children.push(child)
    }

    pub fn set_drawable(&mut self, drawable: Option<Box<dyn Drawable>>) {
        self.drawable = drawable;
    }

    pub fn add_component<C: Component + 'static>(&mut self) {
        let mut comp = Box::new(C::new());
        comp.init();

        self.components.push(Rc::new(RefCell::new(comp)));
    }

    pub fn get_component<C: Component + 'static>(
        &mut self,
    ) -> Option<Rc<RefCell<Box<dyn Component>>>> {
        self.components
            .iter()
            .find(|&c| c.type_id() == TypeId::of::<Rc<RefCell<Box<C>>>>())
            .cloned()
    }
}
