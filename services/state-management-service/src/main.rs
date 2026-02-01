use anyhow::Result;
use state_management_service::{StateManagementService, Config};
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::init();

    info!("Starting State Management Service");

    // Load configuration
    let config = Config::from_env()?;
    
    // Create and start the service
    let service = StateManagementService::new(config).await?;
    
    match service.run().await {
        Ok(_) => info!("State Management Service stopped gracefully"),
        Err(e) => error!("State Management Service error: {}", e),
    }

    Ok(())
}