use log::{self, LevelFilter};
use simplelog::{CombinedLogger, ConfigBuilder, TermLogger, WriteLogger};
use std::fs::{self, OpenOptions};
use std::path::Path;
use time::macros::format_description;

/// Initializes the logger to output logs to both the console and a file in the `logs` directory.
///
/// # Arguments
///
/// * `level` - The log level to set (e.g., Info, Debug, Error).
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Success or error.
pub fn init_logger(level: LevelFilter) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure the `logs` directory exists
    fs::create_dir_all("logs")?;

    // Define the log file path
    let log_file_path = Path::new("logs").join("project_documentation.log");

    // Define a custom time format with milliseconds (3 digits)
    let log_config = ConfigBuilder::new()
        .set_time_format_custom(format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"))
        .set_time_offset_to_local()
        .unwrap()
        .build();

    // Open the log file in append mode
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
        .map_err(|e| {
            log::error!("Failed to open log file: {:?}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })?;

    // Initialize CombinedLogger to log to both file and console
    CombinedLogger::init(vec![
        TermLogger::new(level, log_config.clone(), simplelog::TerminalMode::Stdout, simplelog::ColorChoice::Auto),
        WriteLogger::new(level, log_config, log_file),
    ])
        .map_err(|e| {
            log::error!("Failed to initialize logger: {:?}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })?;

    log::info!("Logger initialized. Logs are being written to {:?}", log_file_path);
    Ok(())
}