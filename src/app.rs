use std::error::Error;

use log::error;
use wgpu::{Device, Queue};
use winit::dpi::{PhysicalSize, Size};
use winit::error::EventLoopError;
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowBuilder;

use crate::components::CameraComp;
use crate::logichooks::LogicHooks;
use crate::renderer::{Renderer, RuntimeRenderer};
use crate::world::World;

pub struct PrematureApp {
    window_builder: Option<WindowBuilder>,
}

pub struct App {
    renderer: RuntimeRenderer,
    world: Box<World>,
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

impl App {
    #[allow(unused)]
    pub fn create(title: &str, width: u32, height: u32) -> PrematureApp {
        PrematureApp {
            window_builder: Some(
                WindowBuilder::new()
                    .with_inner_size(Size::Physical(PhysicalSize { width, height }))
                    //.with_resizable(false)
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
        let mut world =
            unsafe { World::new(renderer.state.device.clone(), renderer.state.queue.clone()) };
        let renderer = renderer.init(&mut world.assets);

        let app = App { world, renderer };

        Ok((event_loop, app))
    }

    pub async fn run(mut self, hooks: LogicHooks) -> Result<(), Box<dyn Error>> {
        let (event_loop, mut app) = self.init_state().await?;

        let renderer = &mut app.renderer;
        let world = &mut app.world;

        if let Some(init) = hooks.init {
            if let Err(e) = init(world, renderer.window()) {
                error!("World init function hook returned: {e}");
            }
        }

        let world_ptr: *mut World = world.as_mut();
        unsafe {
            for obj in (*world_ptr).objects.values_mut() {
                if let Some(ref mut drawable) = obj.drawable {
                    drawable.setup(
                        &renderer.state.device,
                        &renderer.state.queue,
                        &mut *world_ptr,
                        &world
                            .assets
                            .materials
                            .shaders
                            .model_uniform_bind_group_layout,
                        &world
                            .assets
                            .materials
                            .shaders
                            .material_uniform_bind_group_layout,
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
                            if let Some(update_func) = hooks.update {
                                if let Err(e) = update_func(world, renderer.window()) {
                                    error!("Error happened when calling update function hook: {e}");
                                }
                            }

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
                        WindowEvent::Resized(size) => {
                            renderer.state.resize(*size);

                            // For I have sinned, this now becomes my recovery.
                            // I was forgiven, shall it come haunt me later.
                            if let Some(cam) = world.active_camera {
                                if let Some(cam_comp) = cam.get_component::<CameraComp>() {
                                    if let Ok(mut comp) = cam_comp.try_borrow_mut() {
                                        comp.resize(size.width as f32, size.height as f32);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        })?;

        Ok(())
    }
}
