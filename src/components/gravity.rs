use std::cell::RefCell;
use std::rc::Rc;

use nalgebra::Vector3;

use crate::components::Component;
use crate::object::GameObject;

pub struct GravityComp {
    pub acceleration_per_sec: f32,
    velocity: f32,
    max_acceleration: f32,
}

impl Component for GravityComp {
    fn new() -> Self {
        GravityComp {
            acceleration_per_sec: 9.80665,
            velocity: 0.0,
            max_acceleration: 100.0,
        }
    }

    fn init(&mut self, _parent: &mut GameObject) {}

    fn update(&mut self, parent: Rc<RefCell<GameObject>>, delta_time: f32) {
        self.velocity = (self.velocity - self.acceleration_per_sec * delta_time)
            .clamp(-self.max_acceleration, self.max_acceleration);
        let transform = &mut parent.borrow_mut().transform;
        transform.translate(Vector3::new(0.0, self.velocity, 0.0));
    }
}
