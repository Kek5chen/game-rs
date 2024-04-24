use winit::dpi::{PhysicalSize, Size};
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::{EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowBuilder;

pub struct App {
    window_builder: Option<WindowBuilder>,
}

#[allow(unused)]
impl Default for App {
    fn default() -> Self {
        App {
            window_builder: Some(
                WindowBuilder::new()
                    .with_inner_size(Size::Physical(PhysicalSize {
                        width: 800,
                        height: 600,
                    }))
                    .with_title("Default Window"),
            ),
        }
    }
}

impl App {
    #[allow(unused)]
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        App {
            window_builder: Some(
                WindowBuilder::new()
                    .with_inner_size(Size::Physical(PhysicalSize { width, height }))
                    .with_title(title),
            ),
        }
    }

    pub fn run(self) {
        let event_loop = EventLoop::new().unwrap();
        let window = self
            .window_builder
            .unwrap_or_default()
            .build(&event_loop)
            .unwrap();
        event_loop
            .run(|event, window_target| match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                ..
                            },
                        ..
                    } => window_target.exit(),
                    _ => {}
                },
                _ => {}
            })
            .unwrap()
    }
}
