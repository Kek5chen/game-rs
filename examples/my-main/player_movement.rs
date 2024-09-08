use std::cell::RefCell;
use std::rc::Rc;
use log::warn;
use nalgebra::Vector3;
use num_traits::Zero;
use rapier3d::prelude::{vector, LockedAxes};
use winit::keyboard::KeyCode;
use gamers::components::{Component, RigidBodyComponent};
use gamers::object::GameObjectId;
use gamers::world::World;

pub struct PlayerMovement {
	parent: GameObjectId,
	move_speed: f32,
	jump_factor: f32,
	damping_factor: f32,
	rigid_body: Option<Rc<RefCell<Box<RigidBodyComponent>>>>,
	velocity: Vector3<f32>,
	sprint_multiplier: f32,
}

impl Component for PlayerMovement {
	unsafe fn new(parent: GameObjectId) -> Self
	where
		Self: Sized
	{
		PlayerMovement {
			parent,
			move_speed: 300.0,
			damping_factor: 2.0,
			jump_factor: 1.5,
			rigid_body: None,
			velocity: Vector3::zero(),
			sprint_multiplier: 2.0,
		}
	}

	unsafe fn init(&mut self) {
		let rigid = self.get_parent().get_component::<RigidBodyComponent>();
		if let Some(rigid) = rigid.clone() {
			if let Some(rigid) = rigid
				.borrow_mut()
				.get_body_mut()	{
				rigid.set_locked_axes(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Y | LockedAxes::ROTATION_LOCKED_Z, false)
			}
		}
		self.rigid_body = rigid;
	}

	unsafe fn update(&mut self) {
		let mut rigid = match &self.rigid_body {
			None => {
				warn!("Rigid body not set!");
				return;
			}
			Some(rigid) => rigid.borrow_mut()
		};

		let body = match rigid.get_body_mut() {
			None => {
				warn!("Rigid body not in set");
				return;
			}
			Some(rigid) => rigid,
		};

		let world= World::instance();
		let delta_time = world.get_delta_time().as_secs_f32();
		
		self.velocity /= self.damping_factor;

		if world.input.is_key_down(KeyCode::Space) {
			body.apply_impulse(vector![0.0, 0.2 * self.jump_factor, 0.0], true);
		}

		let mut factor = self.move_speed;

		if world.input.is_key_pressed(KeyCode::KeyW) {
			self.velocity += self.parent.transform.forward() * factor;
		}

		if world.input.is_key_pressed(KeyCode::KeyS) {
			self.velocity += -self.parent.transform.forward() * factor;
		}

		if world.input.is_key_pressed(KeyCode::KeyA) {
			self.velocity += -self.parent.transform.right() * factor;
		}

		if world.input.is_key_pressed(KeyCode::KeyD) {
			self.velocity += self.parent.transform.right() * factor;
		}
		
		if world.input.is_key_pressed(KeyCode::ShiftLeft) {
			self.velocity = Vector3::zero();
		}
		
		let mut linvel = body.linvel().clone();
		linvel.x = self.velocity.x;
		linvel.z = self.velocity.z;
		
		body.set_linvel(linvel, true);
	}

	unsafe fn get_parent(&self) -> GameObjectId {
		self.parent
	}
}