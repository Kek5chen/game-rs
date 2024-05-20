use std::any::TypeId;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, SquareMatrix, Vector2, Vector3, Zero};
use itertools::{izip};
use russimp::scene::PostProcess;
use russimp::Vector3D;
use wgpu::{BindGroupLayout, BufferUsages, Device, IndexFormat, RenderPass, RenderPipeline};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::components::{CameraComp, Component};
use crate::components::camera::CameraData;
use crate::drawable::Drawable;
use crate::transform::Transform;

pub struct ObjectRuntimeData {
    vertices_buf: wgpu::Buffer,
    indices_buf: Option<wgpu::Buffer>,
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

    fn update(&mut self, parent: Rc<RefCell<GameObject>>, queue: &Queue) {
        todo!()
    }

    fn draw<'a, 'b>(
        &'a self,
        rpass: &mut RenderPass<'b>,
        pipeline: &RenderPipeline,
        bind_group: &BindGroupLayout,
    ) where
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
    fn setup(&mut self, device: &Device) {
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

        self.runtime_data = Some(ObjectRuntimeData {
            vertices_buf: v_buffer,
            indices_buf: i_buffer,
        });
    }

    fn draw<'a, 'b>(
        &'a self,
        rpass: &mut RenderPass<'b>,
        pipeline: &RenderPipeline,
        bind_group: &BindGroupLayout,
    ) where
        'a: 'b,
    {
        let runtime_data = self
            .runtime_data
            .as_ref()
            .expect("Runtime data should have been setup before calling draw on an object.");
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

pub struct GameObject {
    pub name: String,
    pub children: Vec<Rc<RefCell<GameObject>>>,
    pub transform: Transform,
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
        comp.init(self);

        self.components.push(Rc::new(RefCell::new(comp)));
    }

    pub fn get_component<C: Component + 'static>(&mut self) -> Option<Rc<RefCell<Box<C>>>> {
        let comp = self
            .components
            .iter()
            .find(|&c| c.borrow().as_ref().type_id() == TypeId::of::<CameraComp>())
            .cloned();
        unsafe { std::mem::transmute::<_, Option<Rc<RefCell<Box<C>>>>>(comp) }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ModelData {
    model_mat: Matrix4<f32>,
    mvp_mat: Matrix4<f32>,
}

impl ModelData {
    pub fn empty() -> Self {
        ModelData {
            model_mat: Matrix4::identity(),
            mvp_mat: Matrix4::identity(),
        }
    }

    pub fn update(&mut self, object: &mut GameObject, camera_data: &CameraData) {
        self.model_mat = *object.transform.full_matrix();
        self.mvp_mat = camera_data.proj_view_mat * self.model_mat;
    }
}

unsafe impl Zeroable for ModelData {}

unsafe impl Pod for ModelData {}
