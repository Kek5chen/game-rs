use nalgebra::{UnitQuaternion, Vector3};
use winit::keyboard::KeyCode;
use crate::components::Component;
use crate::object::GameObjectId;
use crate::world::World;

pub struct FreecamController {
	move_speed: f32,
	look_sensitivity: f32,
	parent: GameObjectId,
	yaw: f32,
	pitch: f32,
}

impl Component for FreecamController {
	unsafe fn new(parent: GameObjectId) -> Self
	where
		Self: Sized,
	{
		FreecamController {
			move_speed: 10.0f32,
			look_sensitivity: 0.1f32,
			parent,
			yaw: 0.0,
			pitch: 0.0,
		}
	}

	unsafe fn update(&mut self) {
		let delta_time = World::instance().get_delta_time().as_secs_f32();
		let transform = &mut self.get_parent().transform;
		
		let input = &World::instance().input;

		let mouse_delta = input.get_mouse_delta(); 
		self.yaw += mouse_delta.x * self.look_sensitivity * delta_time;
		self.pitch += mouse_delta.y * self.look_sensitivity * delta_time;

		self.pitch = self.pitch.clamp(-89.0f32, 89.0f32);

		let yaw_rotation = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), self.yaw.to_radians());
		let pitch_rotation = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), self.pitch.to_radians());
		let rotation = pitch_rotation * yaw_rotation;

		transform.set_local_rotation(rotation);

		let mut direction = Vector3::zeros();
		if input.is_key_pressed(KeyCode::KeyW) {
			direction += transform.forward();
		}
		if input.is_key_pressed(KeyCode::KeyS) {
			direction -= transform.forward();
		}
		if input.is_key_pressed(KeyCode::KeyA) {
			direction -= transform.right();
		}
		if input.is_key_pressed(KeyCode::KeyD) {
			direction += transform.right();
		}
		if input.is_key_pressed(KeyCode::Space) {
			direction += Vector3::new(0.0, 1.0, 0.0);
		}
		if input.is_key_pressed(KeyCode::ControlLeft) {
			direction += Vector3::new(0.0, -1.0, 0.0);
		}

		let move_speed = if input.is_key_pressed(KeyCode::ShiftLeft) {
			self.move_speed * 10.0
		} else {
			self.move_speed
		};
		
		if direction != Vector3::zeros() {
			direction = direction.normalize();
			transform.translate(direction * move_speed * delta_time);
		}
	}

	unsafe fn get_parent(&self) -> GameObjectId {
		self.parent
	}
}
