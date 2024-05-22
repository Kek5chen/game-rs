use std::error::Error;

use log::error;
use wgpu::Device;
use winit::dpi::{PhysicalSize, Size};
use winit::error::EventLoopError;
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowBuilder;

use crate::logichooks::LogicHooks;
use crate::renderer::{Renderer, RuntimeRenderer};
use crate::world::World;

pub struct PrematureApp {
    window_builder: Option<WindowBuilder>,
}

pub struct App<'a> {
    renderer: RuntimeRenderer,
    world: World<'a>,
}

#[allow(unused)]
impl Default for PrematureApp {
    fn default() -> PrematureApp {
        PrematureApp {
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

impl<'a> App<'a> {
    #[allow(unused)]
    pub fn create(title: &str, width: u32, height: u32) -> PrematureApp {
        PrematureApp {
            window_builder: Some(
                WindowBuilder::new()
                    .with_inner_size(Size::Physical(PhysicalSize { width, height }))
                    .with_resizable(false)
                    .with_title(title),
            ),
        }
    }
}

impl PrematureApp {
    async fn init_state(&mut self) -> Result<(EventLoop<()>, App), Box<dyn Error>> {
        let event_loop = match EventLoop::new() {
            Err(EventLoopError::NotSupported(_)) => {
                return Err("No graphics backend found that could be used.".into())
            }
            e => e?,
        };
        let window = self
            .window_builder
            .take()
            .unwrap()
            .build(&event_loop)
            .unwrap();

        let renderer = Renderer::new(window).await;

        // TODO: idk if this is safe. maybe?
        //  edit: probably not :> but it works
        let device: *const Device = &renderer.state.device;
        let mut world = unsafe { World::new(&*device) };
        let renderer = renderer.init(&mut world.assets);

        let app = App { world, renderer };

        Ok((event_loop, app))
    }

    pub async fn run(mut self, hooks: LogicHooks) -> Result<(), Box<dyn Error>> {
        let (event_loop, mut app) = self.init_state().await?;

        let renderer = &mut app.renderer;
        let world = &mut app.world;

        if let Some(init) = hooks.init {
            if let Err(e) = init(world) {
                error!("World init function hook returned: {e}");
            }
        }

        let world_ptr: *mut World = world;
        unsafe {
            for obj in &(*world_ptr).objects {
                if let Some(ref mut drawable) = obj.borrow_mut().drawable {
                    drawable.setup(
                        &mut *world_ptr,
                        &world
                            .assets
                            .materials
                            .shaders
                            .model_uniform_bind_group_layout,
                    )
                }
            }
        }

        event_loop.run(move |event, window_target| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == renderer.window().id() => {
                if !renderer.state.input(event) {
                    match event {
                        WindowEvent::RedrawRequested => {
                            world.update();
                            renderer.state.update();
                            if !renderer.render_world(world) {
                                window_target.exit();
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
        })?;

        Ok(())
    }
}
