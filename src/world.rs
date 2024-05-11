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
            transform: TransformComp::new(),
            drawable: None,
            components: vec![],
        };

        self.objects.push(Rc::new(RefCell::new(obj)));
        self.objects.last().cloned().unwrap()
    }

    pub fn add_child(&mut self, obj: Rc<RefCell<GameObject>>) {
        self.children.push(obj)
    }

    pub fn update(&mut self) {
        // AHAHHAHA FUCKING HELL!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! FUCK YOUUUU BORRWOWW CHECKECHERERRR
        let objects = self.objects.clone();
        let obj_comps: Vec<(Rc<RefCell<GameObject>>, Vec<Rc<RefCell<Box<dyn Component>>>>)> = objects
            .into_iter()
            .map(|o| {
                let comps = o.borrow_mut().components.clone();
                (o, comps)
            })
            .collect();
        for (obj, comps) in obj_comps {
            for comp in comps {
                comp.borrow_mut().update(obj.clone())
            }
        }
    }
}
