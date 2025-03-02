use std::fs;

/// Initializes the logger configured by log4rs.
///
/// The logging configuration is loaded from `log4rs.yml`.
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Success or error.
pub fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
    // Ensure the logs directory exists
    fs::create_dir_all("logs")?;

    // Load the log4rs configuration from the YAML file
    let config = log4rs::config::load_config_file("log4rs.yml", Default::default())?;

    // Initialize the logger with the loaded configuration
    log4rs::init_config(config)?;

    log::info!("Logger initialized with log4rs configuration.");
    Ok(())
}