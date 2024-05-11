use crate::components::Component;
use crate::object::GameObject;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

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
        parent.borrow_mut().transform.pos.y -= 0.001;
        println!("All comps: {:?}", &parent.borrow().components)
    }
}
