fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(
        &[
            "./src/controller.proto",
            "./src/agent.proto",
            "./src/scheduler.proto",
            "./src/network.proto",
        ],
        &["./src/"],
    )?;
    Ok(())
}
