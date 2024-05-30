use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::time::{Duration, Instant};

use log::info;
use wgpu::{Device, Queue};

use crate::asset_management::AssetManager;
use crate::components::CameraComp;
use crate::object::GameObject;
use crate::physics::simulator::PhysicsSimulator;
use crate::transform::Transform;

static mut G_WORLD: *mut World = std::ptr::null_mut();

pub struct World {
    pub objects: Vec<Rc<RefCell<Box<GameObject>>>>,
    pub children: Vec<Rc<RefCell<Box<GameObject>>>>,
    pub active_camera: Option<Weak<RefCell<Box<GameObject>>>>,
    pub assets: AssetManager,
    pub physics: PhysicsSimulator,
    last_frame_time: Instant,
}

impl World {
    pub unsafe fn new(device: Rc<Device>, queue: Rc<Queue>) -> Box<World> {
        let mut world = Box::new(World {
            objects: vec![],
            children: vec![],
            active_camera: None,
            assets: AssetManager::new(device, queue),
            last_frame_time: Instant::now(),
            physics: PhysicsSimulator::default(),
        });

        // create a second mutable reference so G_WORLD can be used in (~un~)safe code
        G_WORLD = world.as_mut();

        world
    }

    // TODO: make this an option later when it's too late
    pub fn instance() -> &'static mut World {
        unsafe {
            if G_WORLD.is_null() {
                panic!("G_WORLD has not been initialized");
            }
            &mut *G_WORLD
        }
    }

    pub fn new_object(&mut self, name: &str) -> Rc<RefCell<Box<GameObject>>> {
        let obj = Box::new(GameObject {
            name: name.to_owned(),
            children: vec![],
            transform: Transform::new(),
            drawable: None,
            components: vec![],
        });

        self.objects.push(Rc::new(RefCell::new(obj)));
        self.objects.last().cloned().unwrap()
    }

    pub fn new_camera(&mut self) -> Rc<RefCell<Box<GameObject>>> {
        let obj = self.new_object("Camera");

        obj.borrow_mut().add_component::<CameraComp>();

        if self.active_camera.is_none() {
            self.active_camera = Some(Rc::<RefCell<Box<GameObject>>>::downgrade(&obj));
        }
        obj
    }

    pub fn add_child(&mut self, obj: Rc<RefCell<Box<GameObject>>>) {
        self.children.push(obj)
    }

    pub fn update(&mut self) {
        // i've grown wiser
        unsafe {
            for object in &self.objects {
                let object_ptr = object.as_ptr();
                for comp in &(*object_ptr).components {
                    let comp_ptr = comp.as_ptr();
                    (*comp_ptr).update();
                }
            }
            self.physics.step();
        }

        self.tick_delta_time();
    }

    pub fn print_objects(&self) {
        info!("{} game objects in world.", self.objects.len());
        Self::print_objects_rec(&self.children, 0)
    }

    pub fn print_objects_rec(children: &Vec<Rc<RefCell<Box<GameObject>>>>, i: i32) {
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
