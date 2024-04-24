mod app;

use std::error::Error;
use crate::app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    App::default().run();

    Ok(())
}
