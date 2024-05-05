use std::cell::RefCell;
use std::rc::Rc;
use crate::object::GameObject;

pub struct World<'world> {
    objects: Vec<Rc<RefCell<GameObject<'world>>>>,
    children: Vec<Rc<RefCell<GameObject<'world>>>>,
}

impl<'world> World<'world> {
    pub fn new() -> World<'world> {
        World {
            objects: vec![],
            children: vec![],
        }
    }
    
    pub fn new_object(&mut self, name: &'world str) -> Rc<RefCell<GameObject<'world>>> {
        let obj = GameObject {
            name,
            children: vec![],
        };
        
        self.objects.push(Rc::new(RefCell::new(obj)));
        self.objects.last().cloned().unwrap()
    }
    
    pub fn add_child(&mut self, obj: Rc<RefCell<GameObject<'world>>>) {
        self.children.push(obj)
    }
}