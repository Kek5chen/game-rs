use crate::utils::math::QuaternionEuler;
use rapier3d::prelude::*;

use crate::components::Component;
use crate::object::GameObjectId;
use crate::world::World;

pub struct RigidBodyComponent {
    parent: GameObjectId,
    pub body_handle: RigidBodyHandle,
}

impl Component for RigidBodyComponent {
    unsafe fn new(parent: GameObjectId) -> Self {
        let initial_translation = parent.transform.position();
        let initial_rotation = parent.transform.rotation().euler_vector();
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(initial_translation)
            .rotation(initial_rotation)
            .build();

        let body_handle = World::instance().physics.rigid_body_set.insert(rigid_body);

        RigidBodyComponent {
            parent,
            body_handle,
        }
    }

    unsafe fn init(&mut self) {}

    unsafe fn late_update(&mut self) {
        let rb = World::instance()
            .physics
            .rigid_body_set
            .get_mut(self.body_handle);
        if let Some(rb) = rb {
            rb.set_translation(self.parent.transform.position(), false);
            rb.set_rotation(self.parent.transform.rotation(), false);
        } else {
            todo!("de-synced - remake_rigid_body();")
        }
    }

    unsafe fn post_update(&mut self) {
        let rb = World::instance()
            .physics
            .rigid_body_set
            .get_mut(self.body_handle);
        if let Some(rb) = rb {
            if rb.is_dynamic() {
                self.get_parent().transform.set_position(*rb.translation());
                self.get_parent().transform.set_rotation(*rb.rotation());
            }
        }
    }

    unsafe fn get_parent(&self) -> GameObjectId {
        self.parent
    }
}

impl RigidBodyComponent {
    pub fn get_body(&self) -> Option<&RigidBody> {
        World::instance()
            .physics
            .rigid_body_set
            .get(self.body_handle)
    }

    pub fn get_body_mut(&mut self) -> Option<&mut RigidBody> {
        World::instance()
            .physics
            .rigid_body_set
            .get_mut(self.body_handle)
    }
}
