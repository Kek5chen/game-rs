use std::cell::RefCell;
use std::rc::Rc;

use cgmath::{Deg, Vector3};

use crate::components::Component;
use crate::object::GameObject;

pub struct RotateComponent {
    rotate_speed: Deg<f32>,
    iteration: u64,
}

impl Component for RotateComponent {
    fn new() -> Self
    where
        Self: Sized,
    {
        RotateComponent {
            rotate_speed: Deg(5.0),
            iteration: 0,
        }
    }

    fn init(&mut self, parent: &mut GameObject) {}

    fn update(&mut self, parent: Rc<RefCell<GameObject>>, delta_time: f32) {
        let transform = &mut parent.borrow_mut().transform;
        let y_rot = transform.rotation().y + self.rotate_speed.0 * delta_time;
        transform.set_rotation(Vector3::new(
            (self.iteration as f32 / 500.0).sin() * 100.0,
            y_rot,
            0.0,
        ));
        self.iteration += 1;
    }
}
