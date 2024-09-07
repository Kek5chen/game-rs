use std::any::TypeId;
use std::cell::RefCell;
use std::mem;
use std::ops::{AddAssign, Deref, DerefMut};
use std::rc::Rc;

use bytemuck::{Pod, Zeroable};
use nalgebra::Matrix4;

use crate::components::Component;
use crate::drawable::Drawable;
use crate::hacks;
use crate::transform::Transform;
use crate::world::World;

#[derive(Debug, Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Hash)]
#[repr(transparent)]
pub struct GameObjectId(pub usize);

#[allow(dead_code)]
impl GameObjectId {
    pub(crate) fn exists(&self) -> bool {
        World::instance().objects.contains_key(self)
    }
}

impl AddAssign<usize> for GameObjectId {
    fn add_assign(&mut self, other: usize) {
        self.0 += other;
    }
}

impl Deref for GameObjectId {
    type Target = Box<GameObject>;

    fn deref(&self) -> &Self::Target {
        World::instance().get_object(self).unwrap()
    }
}

impl DerefMut for GameObjectId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        World::instance().get_object_mut(self).unwrap()
    }
}

pub struct GameObject {
    pub id: GameObjectId,
    pub name: String,
    pub children: Vec<GameObjectId>,
    pub parent: Option<GameObjectId>,
    pub transform: Transform,
    pub drawable: Option<Box<dyn Drawable>>,
    pub components: Vec<Rc<RefCell<Box<dyn Component>>>>,
}

impl GameObject {
    pub fn add_child(&mut self, mut child: GameObjectId) {
        // TODO: Make the children know who it's owned by because of circling references
        self.children.push(child);
        child.parent = Some(self.id);
    }

    pub fn set_drawable(&mut self, drawable: Option<Box<dyn Drawable>>) {
        self.drawable = drawable;
    }

    pub fn add_component<'b, C: Component + 'static>(&mut self) -> &'b mut C {
        unsafe {
            let mut comp: Box<dyn Component> = Box::new(C::new(self.id));
            let comp_inner_ptr: hacks::FatPtr<C> =
                mem::transmute(comp.as_mut() as *mut dyn Component);
            let comp_inner_ref: &mut C = &mut *comp_inner_ptr.data;

            comp.init();

            let comp: Rc<RefCell<Box<dyn Component>>> = Rc::new(RefCell::new(comp));
            let comp_dyn: Rc<RefCell<Box<dyn Component>>> = comp;

            self.components.push(comp_dyn);

            comp_inner_ref
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
                    mem::transmute(rc_clone)
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

    pub fn update(&mut self, object: GameObjectId, outer_transform: &Matrix4<f32>) {
        self.model_mat =
            outer_transform * object.transform.full_matrix().to_homogeneous();
    }
}

unsafe impl Zeroable for ModelData {}

unsafe impl Pod for ModelData {}
