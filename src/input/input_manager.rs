use std::collections::HashMap;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

pub type KeyState = ElementState;

pub struct InputManager {
    key_states: HashMap<KeyCode, KeyState>,
    key_just_updated: Vec<KeyCode>,
    button_states: HashMap<MouseButton, ElementState>,
    button_just_updated: Vec<MouseButton>,
    mouse_wheel_delta: f64,
}

#[allow(unused)]
impl InputManager {
    pub fn new() -> InputManager {
        InputManager {
            key_states: HashMap::default(),
            key_just_updated: Vec::new(),
            button_states: HashMap::default(),
            button_just_updated: Vec::new(),
            mouse_wheel_delta: 0.0,
        }
    }

    pub(crate) fn process_event(&mut self, window_event: &WindowEvent) {
        match window_event {
            WindowEvent::KeyboardInput { event, .. } => match event.physical_key {
                PhysicalKey::Code(code) => {
                    if !event.state.is_pressed() || self.key_states.get(&code).is_some_and(|state| !state.is_pressed()) {
                        self.key_just_updated.push(code);
                    }
                    
                    self.key_states.insert(code.clone(), event.state);
                }
                _ => {}
            },
            WindowEvent::CursorMoved { .. } => {}
            WindowEvent::MouseWheel { delta, .. } => {
                let y = match delta {
                    MouseScrollDelta::LineDelta(_, y) => *y as f64,
                    MouseScrollDelta::PixelDelta(pos) => pos.y,
                };
                self.mouse_wheel_delta += y;
            }
            WindowEvent::MouseInput { button, state, .. } => {
                if !state.is_pressed() || self.button_states.get(&button).is_some_and(|state| !state.is_pressed()) {
                    self.button_just_updated.push(button.clone());
                }
                self.button_states.insert(button.clone(), state.clone());
            }
            _ => {}
        }
    }
    
    pub fn get_key_state(&self, key_code: KeyCode) -> KeyState {
        *self.key_states.get(&key_code).unwrap_or(&KeyState::Released)
    }

    pub fn is_key_down(&self, key_code: KeyCode) -> bool {
        self.get_key_state(key_code) == KeyState::Pressed && self.key_just_updated.contains(&key_code)
    }

    pub fn is_key_pressed(&self, key_code: KeyCode) -> bool {
        self.get_key_state(key_code) == KeyState::Pressed
    }
    
    pub fn is_key_released(&self, key_code: KeyCode) -> bool {
        self.get_key_state(key_code) == KeyState::Released && self.key_just_updated.contains(&key_code)
    }

    pub fn is_key_up(&self, key_code: KeyCode) -> bool {
        self.get_key_state(key_code) == KeyState::Released
    }
    
    pub fn set_mouse_state(&self) {
        //World::instance().
    }
    
    pub fn get_button_state(&self, button: MouseButton) -> ElementState {
        *self.button_states.get(&button).unwrap_or(&ElementState::Released)
    }
    
    pub fn is_button_down(&self, button: MouseButton) -> bool {
        self.get_button_state(button) == ElementState::Pressed && self.button_just_updated.contains(&button)
    }

    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        self.get_button_state(button) == ElementState::Pressed
    }

    pub fn is_button_released(&self, button: MouseButton) -> bool {
        self.get_button_state(button) == ElementState::Released && self.button_just_updated.contains(&button)
    }
    
    pub fn next_frame(&mut self) {
        self.key_just_updated.clear();
        self.button_just_updated.clear();
    }
}