mod config;
mod file_processor;
mod markdown_generator;
mod logger;
mod language;

use std::fs;
use std::path::{Path, PathBuf};

/// The main entry point of the project documentation generator.
///
/// This function initializes the logger, loads language definitions, processes project configurations,
/// generates Markdown documentation, and writes the output to specified files.
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Success or error.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger with the configuration from log4rs.yml
    logger::init_logger()?;

    log::info!("Starting to generate project documentation...");

    // Load common language definitions from the `languages.yml` file
    let languages_path = PathBuf::from("languages.yml");
    let languages = language::load_languages(&languages_path)?;
    log::info!("Loaded language definitions from languages.yml");

    // Iterate through all project configuration files in the "projects" directory
    let projects_dir = Path::new("projects");
    for entry in fs::read_dir(projects_dir)? {
        let entry = entry?;
        let config_path = entry.path();
        if config_path.is_file() && config_path.extension().unwrap_or_default() == "yml" {
            log::info!("Processing project configuration: {}", config_path.display());

            // Load the project configuration from the YAML file
            let config = config::Config::load(&config_path)?;
            log::info!("Loaded project configuration: {}", config.project_name);

            // Get the project root directory
            let project_root = Path::new(&config.project_path);

            // Process files and directories specified in the configuration
            let files = file_processor::process_files(&config.project_path, &config.files, &config.directories)?;
            log::info!("Processed files for project: {}", config.project_name);

            // Generate Markdown content for the project documentation
            let markdown_content = markdown_generator::generate_markdown(&config.project_name, files, &languages, project_root);

            // Write the generated Markdown content to the output file
            fs::create_dir_all("output")?;
            let output_path = Path::new("output").join(&config.output_file);
            fs::write(&output_path, markdown_content)?;
            log::info!("Generated documentation for project: {} -> {}", config.project_name, output_path.display());
        }
    }

    log::info!("Project documentation generation complete.");
    Ok(())
}