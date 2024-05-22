use std::cell::RefCell;
use std::rc::Rc;

use cgmath::Matrix4;
use wgpu::{BindGroupLayout, Device, IndexFormat, Queue, RenderPass};

use crate::asset_management::materialmanager::RuntimeMaterial;
use crate::asset_management::mesh::{Mesh, RuntimeMesh};
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
    fn setup(
        &mut self,
        device: &Device,
        queue: &Queue,
        world: &mut World,
        model_uniform_bind_group_layout: &BindGroupLayout,
        material_uniform_bind_group_layout: &BindGroupLayout,
    ) {
        unsafe {
            let world: *mut World = world;
            (*world)
                .assets
                .meshes
                .init_runtime_mesh(self.mesh, model_uniform_bind_group_layout);
            let mesh: *const Box<Mesh> = (*world)
                .assets
                .meshes
                .get_raw_mesh(self.mesh)
                .expect("Normal mesh should be set");

            for (mat_id, _) in &(*mesh).material_ranges {
                (*world).assets.materials.init_runtime_material(
                    &mut *world,
                    queue,
                    *mat_id,
                    material_uniform_bind_group_layout,
                );
            }
        }
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

        let mesh: *const Box<Mesh> = world
            .assets
            .meshes
            .get_raw_mesh(self.mesh)
            .expect("Normal mesh should be set");

        rpass.set_vertex_buffer(0, (*runtime_mesh).data.vertices_buf.slice(..));
        rpass.set_bind_group(1, &(*runtime_mesh).data.model_bind_group, &[]);
        if let Some(i_buffer) = (*runtime_mesh).data.indices_buf.as_ref() {
            rpass.set_index_buffer(i_buffer.slice(..), IndexFormat::Uint32);
            rpass.draw_indexed(0..(*runtime_mesh).data.indices_num as u32, 0, 0..1);
        } else {
            for (mat_id, range) in &(*mesh).material_ranges {
                let material: *const RuntimeMaterial = world
                    .assets
                    .materials
                    .get_runtime_material(*mat_id)
                    .unwrap();

                rpass.set_bind_group(2, &(*material).bind_group, &[]);
                rpass.draw(range.clone(), 0..1);
            }
        }
    }
}
