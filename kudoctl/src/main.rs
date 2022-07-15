use log::trace;
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    trace!("Logger initialized");

    let client = reqwest::Client::new();

    trace!("reqwest Client initialized");
    Ok(())
}
