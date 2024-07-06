mod api;
mod config;
mod cache;
mod errors;
mod models;
mod program;
mod database;
mod metrics;
use std::process::exit;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), anyhow::Error> {
    let config = config::Config::parse()?;
    if let Err(error) = program::run(&config).await {
        eprintln!("Fatal Error: {}", error);
        exit(1);
    }
    Ok(())
}

