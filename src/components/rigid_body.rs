use nalgebra::Vector3;
use rapier3d::prelude::*;

use crate::components::{Component};
use crate::object::GameObject;
use crate::world::World;

pub struct RigidBodyComponent {
    parent: *mut GameObject,
    pub body_handle: RigidBodyHandle,
    i: u32,
}

impl Component for RigidBodyComponent {
    unsafe fn new(parent: *mut GameObject) -> Self {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(*(*parent).transform.position())
            .rotation((*(*parent).transform.rotation()).map(|rad| rad.to_degrees()))
            .build();

        let body_handle = World::instance().physics.rigid_body_set.insert(rigid_body);

        RigidBodyComponent {
            parent,
            body_handle,
            i: 0,
        }
    }

    unsafe fn init(&mut self) {
        let rb = World::instance().physics.rigid_body_set.get_mut(self.body_handle);
        if let Some(rb) = rb {
        }
    }

    unsafe fn update(&mut self) {
        self.i += 1;
        let rb = World::instance().physics.rigid_body_set.get_mut(self.body_handle);
        if let Some(rb) = rb {
            self.get_parent().transform.set_position(*rb.translation());
            let rot = rb.rotation().euler_angles();
            self.get_parent().transform.set_rotation(Vector3::new(rot.0.to_degrees(), rot.1.to_degrees(), rot.2.to_degrees()));
        }
    }

    unsafe fn get_parent(&self) -> &mut GameObject {
        &mut *self.parent
    }
}

