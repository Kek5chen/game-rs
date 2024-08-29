use std::cell::RefCell;
use std::collections::VecDeque;
use std::error::Error;
use std::sync::Mutex;

use env_logger::Env;
use log::{error, LevelFilter};
use nalgebra::Vector3;
use winit::window::Window;

use crate::app::App;
use crate::asset_management::mesh::Mesh;
use crate::buffer::{CUBE, CUBE_INDICES};
use crate::components::RotateComponent;
use crate::logichooks::LogicHooks;
use crate::mesh_renderer::MeshRenderer;
use crate::scene_loader::SceneLoader;
use crate::world::World;

mod app;
mod asset_management;
mod buffer;
mod components;
mod drawable;
mod hacks;
mod logichooks;
mod mesh_renderer;
mod object;
mod physics;
mod renderer;
mod scene_loader;
mod state;
mod transform;
mod world;
mod utils;
mod input;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let log_env = Env::new().filter("RUST_LOG");
    env_logger::builder()
        .parse_default_env() // Default env
        .filter_level(LevelFilter::Info) // Use at least info level
        .parse_env(log_env) // Or override with whatever env says
        .init();

    let mut app = App::create("game-rs", 800, 600);
    app.with_init(Some(funnyinit));
    app.with_update(Some(update));
    
    if let Err(e) = app.run().await {
        error!("{e}");
    }

    Ok(())
}

fn init(world: &mut World, _window: &Window) -> Result<(), Box<dyn Error>> {
    let obj2 = SceneLoader::load(world, "testmodels/parenting_and_object_types.fbx")?;
    // I know that the following code is broken regularly. And I have an explanation for this:
    // I never update the example because I make more sophisticated examples to test.
    // And I'm too lazy to delete my code, update the example, and then put my test code back.
    // Maybe some time in the future I'll actually split this into a library. But for now,
    // have some broken code :)
    let mut obj1 = world.new_object("Mow");
    let mut camera = world.new_camera();

    camera
        .transform
        .set_position(Vector3::new(0.0, 1.0, 50.0));

    obj2.transform.set_uniform_scale(0.03);
    obj2.add_component::<RotateComponent>();
    obj1.add_child(obj2);
    world.add_child(obj1);
    world.add_child(camera);

    world.print_objects();

    Ok(())
}

static LAST_FRAME_TIMES: Mutex<RefCell<VecDeque<f32>>> = Mutex::new(RefCell::new(VecDeque::new()));
const RUNNING_SIZE: usize = 60;

fn update(world: &mut World, window: &Window) -> Result<(), Box<dyn Error>> {
    let last_times = LAST_FRAME_TIMES.lock()?;
    let mut last_times = last_times.borrow_mut();

    let frame_time = world.get_delta_time().as_secs_f32();
    if last_times.len() >= RUNNING_SIZE {
        last_times.pop_front();
    }
    last_times.push_back(frame_time);

    let mean_delta_time: f32 = last_times.iter().sum::<f32>() / last_times.len() as f32;
    window.set_title(&format!(
        "{} - v.{} - built on {} at {} - FPS: [ {} ] #{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("BUILD_DATE"),
        env!("BUILD_TIME"),
        (1.0 / mean_delta_time) as u32,
        env!("GIT_HASH"),
    ));

    Ok(())
}
