use nalgebra::{UnitQuaternion, Vector3};
use rand::random;

use crate::components::Component;
use crate::object::GameObjectId;
use crate::world::World;

pub struct RotateComponent {
    rotate_speed: f32,
    iteration: f32,
    parent: GameObjectId,
    y_rot: f32,
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
            y_rot: 0.0,
        }
    }

    unsafe fn update(&mut self) {
        let transform = &mut self.get_parent().transform;
        let delta_time = World::instance().get_delta_time().as_secs_f32();

        let x_angle_radians = (self.iteration / 100.0).sin() * 45.0f32.to_radians();
        let x_rotation = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), x_angle_radians);

        self.y_rot += self.rotate_speed.to_radians() * delta_time;
        let y_rotation = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), self.y_rot);
        
        let combined_rotation = y_rotation * x_rotation;

        transform.set_rotation(combined_rotation);
        self.iteration += 1.0;
    }

    unsafe fn get_parent(&self) -> GameObjectId {
        self.parent
    }
}
