use rapier3d::prelude::*;

use crate::asset_management::meshmanager::MeshId;
use crate::components::{Component, RigidBodyComponent};
use crate::object::GameObject;
use crate::world::World;

pub enum MeshColliderType {
    Sphere(f32),         // radius
    Cube(f32, f32, f32), // width, height, depth
    Mesh(MeshId),
}

pub struct MeshColliderComponent {
    mesh_type: MeshColliderType,
    parent: *mut GameObject,
    pub phys_handle: ColliderHandle,
    linked_to_body: Option<RigidBodyHandle>,
    collider: Collider,
}

impl Component for MeshColliderComponent {
    unsafe fn new(parent: *mut GameObject) -> Self {
        let mut mesh_type = MeshColliderType::Cube(100.0, 100.0, 100.0);

        let collider = match mesh_type {
            MeshColliderType::Sphere(radius) => ColliderBuilder::ball(radius),
            MeshColliderType::Cube(x, y, z) => ColliderBuilder::cuboid(x, y, z),
            MeshColliderType::Mesh(_) => ColliderBuilder::ball(1.0),
        }
        .density(1.0)
        .restitution(1.0)
        .build();

        let handle = World::instance()
            .physics
            .collider_set
            .insert(collider.clone());

        MeshColliderComponent {
            mesh_type,
            parent,
            phys_handle: handle,
            linked_to_body: None,
            collider,
        }
    }

    unsafe fn init(&mut self) {}

    unsafe fn update(&mut self) {
        if self.linked_to_body.is_none() {
            let body_comp = self.get_parent().get_component::<RigidBodyComponent>();
            if let Some(body_comp) = body_comp {
                self.link_to_rigid_body(body_comp.borrow().body_handle);
            } else {
                World::instance()
                    .physics
                    .collider_set
                    .get_mut(self.phys_handle)
                    .unwrap()
                    .set_translation(*(*self.parent).transform.position());
            }
        }
    }

    unsafe fn get_parent(&self) -> &mut GameObject {
        &mut *self.parent
    }
}

impl MeshColliderComponent {
    pub fn link_to_rigid_body(&mut self, h_body: RigidBodyHandle) {
        let world = World::instance();

        let body = world.physics.rigid_body_set.get(h_body);
        if body.is_none() {
            return;
        }

        // remove old
        world.physics.collider_set.remove(
            self.phys_handle,
            &mut world.physics.island_manager,
            &mut world.physics.rigid_body_set,
            false,
        );

        // insert new
        world.physics.collider_set.insert_with_parent(
            self.collider.clone(),
            h_body,
            &mut world.physics.rigid_body_set,
        );

        self.linked_to_body = Some(h_body);
    }
}
