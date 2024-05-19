use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(unused_imports)]
pub use camera::CameraComp;
#[allow(unused_imports)]
pub use gravity::GravityComp;
#[allow(unused_imports)]
pub use rotate::RotateComponent;

use crate::object::GameObject;

pub mod camera;
pub mod gravity;
pub mod rotate;

pub trait Component: Any {
    fn new() -> Self
    where
        Self: Sized;
    fn init(&mut self, parent: &mut GameObject);
    fn update(&mut self, parent: Rc<RefCell<GameObject>>, delta_time: f32);
}
