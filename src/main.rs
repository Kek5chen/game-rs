mod app;
mod buffer;
mod drawable;
mod object;
mod renderer;
mod state;

use crate::app::App;
use env_logger::Env;
use log::{error, LevelFilter};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let log_env = Env::new().filter("RUST_LOG");
    env_logger::builder()
        .parse_default_env() // Default env
        .filter_level(LevelFilter::Info)// Use at least info level
        .parse_env(log_env) // Or override with whatever env says
        .init();

    if let Err(e) = App::default().run().await {
        error!("{e}")
    }

    Ok(())
}
