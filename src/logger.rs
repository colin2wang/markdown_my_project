use std::fs;
use anyhow::{Context, Result};

/// Initializes the logger configured by log4rs.
///
/// The logging configuration is loaded from `log4rs.yml`.
///
/// # Returns
///
/// * `Result<()>` - Success or error.
pub fn init_logger() -> Result<()> {
    // Ensure the logs directory exists
    fs::create_dir_all("logs")
        .context("Failed to create logs directory")?;

    // Load the log4rs configuration from the YAML file
    let config = log4rs::config::load_config_file("log4rs.yml", Default::default())
        .context("Failed to load log4rs configuration")?;

    // Initialize the logger with the loaded configuration
    log4rs::init_config(config)
        .context("Failed to initialize log4rs")?;

    log::info!("Logger initialized with log4rs configuration.");
    Ok(())
}