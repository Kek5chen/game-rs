use crate::components::Component;
use crate::object::GameObject;
use std::cell::RefCell;
use std::rc::Rc;
use cgmath::Vector3;

pub struct GravityComp {
    acceleration: f32,
    max_acceleration: f32,
}

impl Component for GravityComp {
    fn new() -> Self {
        GravityComp {
            acceleration: 0.0,
            max_acceleration: 100.0,
        }
    }

    fn init(&mut self) {}

    fn update(&mut self, parent: Rc<RefCell<GameObject>>, delta_time: f32) {
        self.acceleration = (self.acceleration - 8.41 * delta_time).clamp(-self.max_acceleration, self.max_acceleration);
        let transform = &mut parent.borrow_mut().transform;
        transform.translate(Vector3::new(0.0, self.acceleration, 0.0));
    }
}
