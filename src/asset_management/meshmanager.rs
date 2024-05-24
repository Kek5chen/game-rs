use std::collections::HashMap;

use wgpu::{BindGroupLayout, Device};

use crate::asset_management::mesh::{Mesh, RuntimeMesh};

pub type MeshId = usize;

pub struct MeshManager<'a> {
    meshes: HashMap<MeshId, (Box<Mesh>, Option<RuntimeMesh>)>,
    next_id: MeshId,
    device: &'a Device,
}

#[allow(dead_code)]
impl<'a> MeshManager<'a> {
    pub(crate) fn new(device: &'a Device) -> MeshManager<'a> {
        MeshManager {
            meshes: HashMap::new(),
            next_id: 0,
            device,
        }
    }

    pub fn add_mesh(&mut self, mesh: Box<Mesh>) -> MeshId {
        let id = self.next_id;

        self.meshes.insert(id, (mesh, None));
        self.next_id += 1;

        id
    }

    pub fn get_mesh_internal_mut(
        &mut self,
        id: MeshId,
    ) -> Option<&mut (Box<Mesh>, Option<RuntimeMesh>)> {
        self.meshes.get_mut(&id)
    }

    pub fn get_raw_mesh(&self, id: MeshId) -> Option<&Box<Mesh>> {
        self.meshes.get(&id).map(|m| &m.0)
    }

    pub fn get_runtime_mesh(&self, id: MeshId) -> Option<&RuntimeMesh> {
        let mesh = self.meshes.get(&id);
        match mesh {
            None => None,
            Some((_, opt_runtime_mesh)) => opt_runtime_mesh.as_ref(),
        }
    }

    pub fn get_runtime_mesh_mut(&mut self, id: MeshId) -> Option<&mut RuntimeMesh> {
        let mesh = self.meshes.get_mut(&id);
        match mesh {
            None => None,
            Some((_, opt_runtime_mesh)) => opt_runtime_mesh.as_mut(),
        }
    }

    pub fn init_runtime_mesh(
        &mut self,
        id: MeshId,
        model_uniform_bind_group_layout: &BindGroupLayout,
    ) {
        self.get_runtime_mesh_or_init(id, model_uniform_bind_group_layout);
    }

    pub fn get_runtime_mesh_or_init(
        &mut self,
        id: MeshId,
        model_uniform_bind_group_layout: &BindGroupLayout,
    ) -> Option<&RuntimeMesh> {
        let mesh = self.meshes.get_mut(&id);
        match mesh {
            None => None,
            Some((mesh, opt_runtime_mesh)) => match opt_runtime_mesh {
                None => {
                    let runtime_mesh =
                        mesh.init_runtime(self.device, model_uniform_bind_group_layout);
                    *opt_runtime_mesh = Some(runtime_mesh);
                    opt_runtime_mesh.as_ref()
                }
                Some(runtime_mesh) => Some(runtime_mesh),
            },
        }
    }
}
