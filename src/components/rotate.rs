use std::cell::RefCell;
use std::rc::Rc;

use cgmath::{Deg, Vector3};
use rand::random;

use crate::components::Component;
use crate::object::GameObject;

pub struct RotateComponent {
    rotate_speed: Deg<f32>,
    iteration: f64,
}

impl Component for RotateComponent {
    fn new() -> Self
    where
        Self: Sized,
    {
        RotateComponent {
            rotate_speed: Deg(50.0),
            iteration: random(), // TODO: is cool but might lead to odd behavior
        }
    }

    fn init(&mut self, _parent: &mut GameObject) {}

    fn update(&mut self, parent: Rc<RefCell<GameObject>>, delta_time: f32) {
        let transform = &mut parent.borrow_mut().transform;
        let y_rot = transform.rotation().y + self.rotate_speed.0 * delta_time;
        transform.set_rotation(Vector3::new(
            (self.iteration as f32 / 100.0).sin() * 45.0,
            y_rot,
            0.0,
        ));
        self.iteration += 1.01;
    }
}
