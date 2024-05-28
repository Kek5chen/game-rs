use std::any::Any;

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

// TODO: resolve unsafe hell
pub trait Component: Any {
    unsafe fn new(parent: *mut GameObject) -> Self
    where
        Self: Sized;
    unsafe fn init(&mut self);
    unsafe fn update(&mut self);

    #[allow(clippy::mut_from_ref)]
    unsafe fn get_parent(&self) -> &mut GameObject;
}
