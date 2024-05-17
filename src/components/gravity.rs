use crate::components::Component;
use crate::object::GameObject;
use std::cell::RefCell;
use std::rc::Rc;
use cgmath::Vector3;

pub struct GravityComp {}

impl Component for GravityComp {
    fn new() -> Self
    where
        Self: Sized,
    {
        GravityComp {}
    }

    fn init(&mut self) {}

    fn update(&mut self, parent: Rc<RefCell<GameObject>>) {
        let transform = &mut parent.borrow_mut().transform;
        transform.translate(Vector3::new(0.0, -0.001, 0.0));
    }
}
