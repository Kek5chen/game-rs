use rapier3d::prelude::*;

use crate::asset_management::meshmanager::MeshId;
use crate::components::{Component, RigidBodyComponent};
use crate::object::GameObject;
use crate::world::World;

#[derive(Clone)]
pub enum ColliderType {
    Sphere(f32),         // radius
    Cube(f32, f32, f32), // width, height, depth
    Mesh(MeshId),
}

pub struct Collider3D {
    coll_type: ColliderType,
    pub phys_handle: ColliderHandle,
    linked_to_body: Option<RigidBodyHandle>,
    collider: Collider,
    parent: *mut GameObject,
}

impl Component for Collider3D {
    unsafe fn new(parent: *mut GameObject) -> Self
    where
        Self: Sized,
    {
        let coll_type = ColliderType::Cube(1.0, 1.0, 1.0);
        let collider = Self::default_collider(&coll_type);
        let phys_handle = World::instance()
            .physics
            .collider_set
            .insert(collider.clone());

        Collider3D {
            coll_type,
            phys_handle,
            linked_to_body: None,
            collider,
            parent,
        }
    }

    unsafe fn init(&mut self) {}

    unsafe fn update(&mut self) {
        if self.linked_to_body.is_none() {
            let body_comp = (*self.parent).get_component::<RigidBodyComponent>();
            if let Some(body_comp) = body_comp {
                self.link_to_rigid_body(Some(body_comp.borrow().body_handle));
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

    pub fn reshape(&mut self, coll_type: ColliderType) {
        let collider = Self::default_collider(&coll_type);

        self.collider = collider;

        let world = World::instance();
        // remove old
        world.physics.collider_set.remove(
            self.phys_handle,
            &mut world.physics.island_manager,
            &mut world.physics.rigid_body_set,
            false,
        );

        // insert new
        self.phys_handle = world.physics.collider_set.insert(self.collider.clone());

        if let Some(h_body) = self.linked_to_body {
            self.link_to_rigid_body(Some(h_body));
        }
    }

    fn default_collider(coll_type: &ColliderType) -> Collider {
        match coll_type {
            ColliderType::Sphere(radius) => ColliderBuilder::ball(*radius),
            ColliderType::Cube(x, y, z) => ColliderBuilder::cuboid(*x, *y, *z),
            ColliderType::Mesh(_) => ColliderBuilder::ball(1.0), // FIXME: actual mesh collider
        }
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
