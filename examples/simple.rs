use august_core::{Core, Config};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();

    // Create configuration
    let config = Config::new();

    // Initialize core
    let mut core = Core::new(config)?;

    // Process command
    let result = core.process("Modify photo from SimTech, apply Ghibli style").await?;
    println!("Result: {}", serde_json::to_string_pretty(&result)?);

    Ok(())
}

