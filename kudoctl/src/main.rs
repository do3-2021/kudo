mod config;
use log::{trace, LevelFilter};
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let level = LevelFilter::Trace;

    env_logger::builder().filter_level(level);
    trace!("Logger initialized");

    let client = reqwest::Client::new();

    trace!("reqwest Client initialized");
    Ok(())
}
