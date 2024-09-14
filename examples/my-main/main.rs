#![feature(trait_upcasting)]

use std::any::Any;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::error::Error;
use std::sync::Mutex;

use env_logger::Env;
use log::{error, LevelFilter};
use nalgebra::Vector3;
use rapier3d::prelude::*;
use winit::window::Window;

use gamers::app::App;
use gamers::components::{Collider3D, RigidBodyComponent};
use gamers::components::collider::MeshShapeExtra;
use gamers::drawables::mesh_renderer::MeshRenderer;
use gamers::scene_loader::SceneLoader;
use gamers::world::World;
use crate::camera_controller::CameraController;
use crate::player_movement::PlayerMovement;

mod camera_controller;
mod player_movement;

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

fn funnyinit(world: &mut World, _window: &Window) -> Result<(), Box<dyn Error>> {
    let mut city = SceneLoader::load(world, "./testmodels/GM Big City/gm_bigcity.obj")?;

    for child in &mut city.children {
        let collider = child.add_component::<Collider3D>();
        let drawable = &child.drawable;
        let renderer = match
            match drawable {
                None => continue,
                Some(renderer) => (renderer.as_ref() as &dyn Any).downcast_ref::<MeshRenderer>(),
            } {
            None => continue,
            Some(renderer) => renderer,
        };

        let collider = collider.get_collider_mut();
        collider.unwrap().set_shape(SharedShape::mesh(renderer.mesh()).unwrap())
    }

    let mut char_controller = world.new_object("CharacterController");
    let mut camera = world.new_camera();

    char_controller
         .transform
         .set_position(Vector3::new(0.0, 100.0, 0.0));
    //obj2.transform.set_uniform_scale(0.03);

    //city.add_component::<Collider3D>().get_collider_mut().unwrap()
    //    .set_shape(SharedShape::halfspace(Unit::new_unchecked(Vector3::new(0.0, 1.0, 0.0))));
    world.add_child(city);

    world.add_child(char_controller);
    char_controller.add_child(camera);
    camera.add_component::<CameraController>();
    let collider = char_controller.add_component::<Collider3D>();
    collider.get_collider_mut().unwrap().set_shape(SharedShape::capsule_y(1.0, 0.25));

    let _rigid_body = char_controller.add_component::<RigidBodyComponent>();
    char_controller.add_component::<PlayerMovement>();

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
    
    world.input.set_mouse_mode(true);

    Ok(())
}
