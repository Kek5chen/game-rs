use nalgebra::Vector3;
use rand::random;

use crate::components::Component;
use crate::object::GameObjectId;
use crate::world::World;

pub struct RotateComponent {
    rotate_speed: f32,
    iteration: f64,
    parent: GameObjectId,
}

impl Component for RotateComponent {
    unsafe fn new(parent: GameObjectId) -> Self
    where
        Self: Sized,
    {
        RotateComponent {
            rotate_speed: 50.0f32,
            iteration: random(), // TODO: is cool but might lead to odd behavior
            parent,
        }
    }

    unsafe fn update(&mut self) {
        let transform = &mut self.get_parent().transform;
        let delta_time = World::instance().get_delta_time().as_secs_f32();
        let y_rot = transform.rotation().y + self.rotate_speed * delta_time;
        transform.set_rotation(Vector3::new(
            (self.iteration as f32 / 100.0).sin() * 45.0,
            y_rot,
            0.0,
        ));
        self.iteration += 1.01;
    }

    unsafe fn get_parent(&self) -> GameObjectId {
        self.parent
    }
}
