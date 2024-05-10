use crate::components::TransformComp;
use crate::object::GameObject;
use std::cell::RefCell;
use std::rc::Rc;

pub struct World {
    pub objects: Vec<Rc<RefCell<GameObject>>>,
    children: Vec<Rc<RefCell<GameObject>>>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: vec![],
            children: vec![],
        }
    }

    pub fn new_object(&mut self, name: &str) -> Rc<RefCell<GameObject>> {
        let obj = GameObject {
            name: name.to_owned(),
            children: vec![],
            transform: TransformComp::default(),
            drawable: None,
        };

        self.objects.push(Rc::new(RefCell::new(obj)));
        self.objects.last().cloned().unwrap()
    }

    pub fn add_child(&mut self, obj: Rc<RefCell<GameObject>>) {
        self.children.push(obj)
    }
}
