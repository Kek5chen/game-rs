use std::cell::RefCell;
use std::rc::Rc;

use cgmath::Matrix4;
use wgpu::{BindGroupLayout, IndexFormat, Queue, RenderPass};

use crate::asset_management::mesh::RuntimeMesh;
use crate::asset_management::meshmanager::MeshId;
use crate::drawable::Drawable;
use crate::object::GameObject;
use crate::world::World;

pub struct MeshRenderer {
    mesh: MeshId,
}

impl MeshRenderer {
    pub fn new(mesh: MeshId) -> Box<MeshRenderer> {
        Box::new(MeshRenderer { mesh })
    }
}

impl Drawable for MeshRenderer {
    fn setup(&mut self, world: &mut World, bind_group_layout: &BindGroupLayout) {
        world
            .assets
            .meshes
            .init_runtime_mesh(self.mesh, bind_group_layout);
    }

    fn update(
        &mut self,
        world: &mut World,
        parent: Rc<RefCell<GameObject>>,
        queue: &Queue,
        outer_transform: &Matrix4<f32>,
    ) {
        let runtime_mesh = world
            .assets
            .meshes
            .get_runtime_mesh_mut(self.mesh)
            .expect("Runtime mesh should be initialized before calling update.");
        runtime_mesh
            .data
            .model_data
            .update(parent.clone(), outer_transform);
        queue.write_buffer(
            &runtime_mesh.data.model_data_buffer,
            0,
            bytemuck::cast_slice(&[runtime_mesh.data.model_data]),
        )
    }

    unsafe fn draw(&self, world: &World, rpass: &mut RenderPass) {
        let runtime_mesh: *const RuntimeMesh = world
            .assets
            .meshes
            .get_runtime_mesh(self.mesh)
            .expect("Runtime mesh should be initialized before calling draw.");

        rpass.set_bind_group(1, &(*runtime_mesh).data.model_bind_group, &[]);
        rpass.set_vertex_buffer(0, (*runtime_mesh).data.vertices_buf.slice(..));
        if let Some(i_buffer) = (*runtime_mesh).data.indices_buf.as_ref() {
            rpass.set_index_buffer(i_buffer.slice(..), IndexFormat::Uint32);
            rpass.draw_indexed(0..(*runtime_mesh).data.indices_num as u32, 0, 0..1);
        } else {
            rpass.draw(0..(*runtime_mesh).data.vertices_num as u32, 0..1)
        }
    }
}
