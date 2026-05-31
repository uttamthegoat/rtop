mod app;
mod config;
mod error;
mod event;
mod input;
mod state;
mod ui;
mod system;
mod services;
mod models;
mod utils;

use anyhow::Result;
use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let config = config::Config::load()?;
    let mut app = App::new(config)?;
    app.run().await?;

    Ok(())
}
