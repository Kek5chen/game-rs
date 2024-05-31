use nalgebra::Vector3;
use rapier3d::prelude::*;

use crate::asset_management::meshmanager::MeshId;
use crate::components::{Component, RigidBodyComponent};
use crate::object::GameObject;
use crate::world::World;

pub struct Collider3D {
    pub phys_handle: ColliderHandle,
    linked_to_body: Option<RigidBodyHandle>,
    parent: *mut GameObject,
}

impl Component for Collider3D {
    unsafe fn new(parent: *mut GameObject) -> Self
    where
        Self: Sized,
    {
        let shape = SharedShape::cuboid(1.0, 1.0, 1.0);
        let collider = Self::default_collider(shape);
        let phys_handle = World::instance()
            .physics
            .collider_set
            .insert(collider.clone());

        Collider3D {
            phys_handle,
            linked_to_body: None,
            parent,
        }
    }

    unsafe fn update(&mut self) {
        if self.linked_to_body.is_none() {
            let body_comp = (*self.parent).get_component::<RigidBodyComponent>();
            if let Some(body_comp) = body_comp {
                self.link_to_rigid_body(Some(body_comp.borrow().body_handle));
                self.get_collider_mut()
                    .unwrap()
                    .set_translation(Vector3::zeros());
            } else {
                let translation = *(*self.parent).transform.position();
                self.get_collider_mut()
                    .unwrap()
                    .set_translation(translation);
            }
        }
    }

    unsafe fn get_parent(&self) -> &mut GameObject {
        &mut *self.parent
    }
}

impl Collider3D {
    pub fn get_collider(&self) -> Option<&Collider> {
        World::instance().physics.collider_set.get(self.phys_handle)
    }

    pub fn get_collider_mut(&mut self) -> Option<&mut Collider> {
        World::instance()
            .physics
            .collider_set
            .get_mut(self.phys_handle)
    }

    fn default_collider(shape: SharedShape) -> Collider {
        ColliderBuilder::new(shape)
        .density(1.0)
        .restitution(1.0)
        .build()
    }

    pub fn link_to_rigid_body(&mut self, h_body: Option<RigidBodyHandle>) {
        let world = World::instance();

        world.physics.collider_set.set_parent(
            self.phys_handle,
            h_body,
            &mut world.physics.rigid_body_set,
        );

        self.linked_to_body = h_body;
    }
}

pub trait MeshShapeExtra<T> {
    fn mesh(mesh: MeshId) -> Option<T>;
    fn mesh_convex_hull(mesh: MeshId) -> Option<SharedShape>;
}

impl MeshShapeExtra<SharedShape> for SharedShape {
    fn mesh(mesh: MeshId) -> Option<SharedShape> {
        let mesh = World::instance().assets.meshes.get_raw_mesh(mesh)?;
        let vertices = mesh.data.make_point_cloud();
        let indices = mesh.data.make_triangle_indices();
        Some(SharedShape::trimesh(vertices, indices))
    }

    fn mesh_convex_hull(mesh: MeshId) -> Option<SharedShape> {
        let mesh = World::instance().assets.meshes.get_raw_mesh(mesh)?;
        let vertices = mesh.data.make_point_cloud();
        SharedShape::convex_hull(&vertices)
    }
}
