use std::any::TypeId;
use std::cell::RefCell;
use std::rc::Rc;

use bytemuck::{Pod, Zeroable};
use nalgebra::Matrix4;

use crate::components::{CameraComp, Component};
use crate::drawable::Drawable;
use crate::transform::Transform;

pub struct GameObject {
    pub name: String,
    pub children: Vec<Rc<RefCell<GameObject>>>,
    pub transform: Transform,
    pub drawable: Option<Box<dyn Drawable>>,
    pub components: Vec<Rc<RefCell<Box<dyn Component>>>>,
}

impl GameObject {
    pub fn add_child(&mut self, child: Rc<RefCell<GameObject>>) {
        // TODO: Make the children know who it's owned by because of circling references
        self.children.push(child)
    }

    pub fn set_drawable(&mut self, drawable: Option<Box<dyn Drawable>>) {
        self.drawable = drawable;
    }

    pub fn add_component<C: Component + 'static>(&mut self) {
        unsafe {
            let mut comp = Box::new(C::new(self));
            comp.init();

            self.components.push(Rc::new(RefCell::new(comp)));
        }
    }

    // FIXME: this works for now but is stupidly fucked up. 
    //   only change this if entity ids are used for Components in the future :>>
    pub fn get_component<C: Component + 'static>(&self) -> Option<Rc<RefCell<Box<C>>>> {
        for component in &self.components {
            let raw_ptr: *const Box<dyn Component> = component.as_ptr();
            let type_id = unsafe { (**raw_ptr).type_id() };

            if type_id == TypeId::of::<C>() {
                return Some(unsafe {
                    let rc_clone = Rc::clone(component);
                    std::mem::transmute(rc_clone)
                });
            }
        }
        None
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ModelData {
    pub model_mat: Matrix4<f32>,
}

impl ModelData {
    pub fn empty() -> Self {
        ModelData {
            model_mat: Matrix4::identity(),
        }
    }

    pub fn update(&mut self, object: Rc<RefCell<GameObject>>, outer_transform: &Matrix4<f32>) {
        self.model_mat = outer_transform * object.borrow_mut().transform.full_matrix();
    }
}

unsafe impl Zeroable for ModelData {}

unsafe impl Pod for ModelData {}
