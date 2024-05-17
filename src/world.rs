use crate::components::{CameraComp, Component, TransformComp};
use crate::object::GameObject;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct World {
    pub objects: Vec<Rc<RefCell<GameObject>>>,
    pub children: Vec<Rc<RefCell<GameObject>>>,
    pub active_camera: Option<Weak<RefCell<GameObject>>>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: vec![],
            children: vec![],
            active_camera: None,
        }
    }

    pub fn new_object(&mut self, name: &str) -> Rc<RefCell<GameObject>> {
        let mut obj = GameObject {
            name: name.to_owned(),
            children: vec![],
            transform: TransformComp::new(),
            drawable: None,
            components: vec![],
        };

        obj.transform.init();

        self.objects.push(Rc::new(RefCell::new(obj)));
        self.objects.last().cloned().unwrap()
    }

    pub fn new_camera(&mut self) -> Rc<RefCell<GameObject>> {
        let obj = self.new_object("Camera");

        obj.borrow_mut().add_component::<CameraComp>();

        if self.active_camera.is_none() {
            self.active_camera = Some(Rc::<RefCell<GameObject>>::downgrade(&obj));
        }
        obj
    }

    pub fn add_child(&mut self, obj: Rc<RefCell<GameObject>>) {
        self.children.push(obj)
    }

    pub fn update(&mut self) {
        // i've grown wiser
        unsafe {
            for object in &self.objects {
                let object_ptr = object.as_ptr();
                for comp in &(*object_ptr).components {
                    let comp_ptr = comp.as_ptr();
                    (*comp_ptr).update(object.clone())
                }
            }
        }
    }
}
