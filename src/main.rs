mod app;
mod buffer;
mod components;
mod drawable;
mod object;
mod renderer;
mod state;
mod world;

use crate::app::App;
use crate::buffer::{CUBE, CUBE_INDICES};
use crate::object::Object3D;
use crate::world::World;
use env_logger::Env;
use log::{error, LevelFilter};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let log_env = Env::new().filter("RUST_LOG");
    env_logger::builder()
        .parse_default_env() // Default env
        .filter_level(LevelFilter::Info) // Use at least info level
        .parse_env(log_env) // Or override with whatever env says
        .init();

    let mut world = World::new();
    let obj1 = world.new_object("Mow");
    let obj2 = world.new_object("Meoow");

    obj2.borrow_mut().set_drawable(Some(Box::new(Object3D::new(
        CUBE.to_vec(),
        Some(CUBE_INDICES.to_vec()),
    ))));
    obj1.borrow_mut().add_child(obj2);
    world.add_child(obj1);

    let app = App::new("game-rs", 800, 600);
    if let Err(e) = app.run(&mut world).await {
        error!("{e}")
    }

    Ok(())
}
