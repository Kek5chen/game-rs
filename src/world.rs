use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::time::{Duration, Instant};

use log::info;
use wgpu::{Device, Queue};

use crate::asset_management::AssetManager;
use crate::components::CameraComp;
use crate::object::GameObject;
use crate::transform::Transform;

pub struct World<'a> {
    pub objects: Vec<Rc<RefCell<GameObject>>>,
    pub children: Vec<Rc<RefCell<GameObject>>>,
    pub active_camera: Option<Weak<RefCell<GameObject>>>,
    pub assets: AssetManager<'a>,
    last_frame_time: Instant,
}

impl<'a> World<'a> {
    pub fn new(device: &'a Device, queue: &Queue) -> World<'a> {
        World {
            objects: vec![],
            children: vec![],
            active_camera: None,
            assets: AssetManager::new(device, queue),
            last_frame_time: Instant::now(),
        }
    }

    pub fn new_object(&mut self, name: &str) -> Rc<RefCell<GameObject>> {
        let obj = GameObject {
            name: name.to_owned(),
            children: vec![],
            transform: Transform::new(),
            drawable: None,
            components: vec![],
        };

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
                    let delta_time = self.get_delta_time().as_secs_f32();
                    (*comp_ptr).update(object.clone(), delta_time)
                }
            }
        }

        self.tick_delta_time();
    }

    pub fn print_objects(&self) {
        info!("{} game objects in world.", self.objects.len());
        Self::print_objects_rec(&self.children, 0)
    }

    pub fn print_objects_rec(children: &Vec<Rc<RefCell<GameObject>>>, i: i32) {
        for child in children {
            info!("{}- {}", "  ".repeat(i as usize), &child.borrow().name);
            Self::print_objects_rec(&child.borrow().children, i + 1);
        }
    }
    
    fn tick_delta_time(&mut self) {
        self.last_frame_time = Instant::now();
    }
    
    pub fn get_delta_time(&self) -> Duration {
        self.last_frame_time.elapsed()
    }
}
