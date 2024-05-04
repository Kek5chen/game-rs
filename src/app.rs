use crate::renderer::Renderer;
use winit::dpi::{PhysicalSize, Size};
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowBuilder;

pub struct App {
    window_builder: Option<WindowBuilder>,
    renderer: Option<Renderer>,
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
            renderer: None,
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
            renderer: None,
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

        self.renderer = Some(Renderer::new(window).await);

        event_loop
    }

    pub async fn run(mut self) {
        let event_loop = self.init_state().await;

        let renderer = self.renderer.as_mut().unwrap();
        
        event_loop
            .run(move |event, window_target| match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == renderer.window().id() => {
                    if !renderer.state.input(event) {
                        match event {
                            WindowEvent::RedrawRequested => {
                                renderer.state.update();
                                match renderer.begin_render() {
                                    Ok(ctx) => {
                                        renderer.end_render(ctx);
                                    }
                                    Err(wgpu::SurfaceError::Lost) => renderer.state.resize(renderer.state.size),
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
                            WindowEvent::Resized(size) => renderer.state.resize(*size),
                            _ => {}
                        }
                    }
                }
                _ => {}
            })
            .unwrap()
    }
}
