pub mod camera;
pub mod gravity;
pub mod transform;

pub use camera::CameraComp;
pub use gravity::GravityComp;
pub use transform::TransformComp;

use crate::object::GameObject;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Component {
    fn new() -> Self
    where
        Self: Sized;
    fn init(&mut self);
    fn update(&mut self, parent: Rc<RefCell<GameObject>>);
}
