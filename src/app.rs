use winit::dpi::{PhysicalSize, Size};
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::{EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowBuilder;

pub struct App {
    window_builder: Option<WindowBuilder>,
    state: Option<State>,
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
            state: None,
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
            state: None,
        }
    }

    async fn init_state(&mut self) -> EventLoop<()> {
        let event_loop = EventLoop::new().unwrap();
        let window = self
            .window_builder
            .take()
            .unwrap()
            .build(&event_loop)
            .unwrap();

        self.state = Some(State::new(window).await);

        event_loop
    }

    pub async fn run(mut self) {
        let event_loop = self.init_state().await;

        let mut state = self.state.unwrap();

        event_loop
            .run(move |event, window_target| match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == state.window().id() => match event {
                    WindowEvent::RedrawRequested => {
                        state.update();
                        match state.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                            Err(wgpu::SurfaceError::OutOfMemory) => window_target.exit(),
                            Err(e) => eprintln!("{:?}", e),
                        }
                    }
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                ..
                            },
                        ..
                    } => window_target.exit(),
                    WindowEvent::Resized(size) => state.resize(*size),
                    _ => {}
                },
                _ => {}
            })
            .unwrap()
    }
}
