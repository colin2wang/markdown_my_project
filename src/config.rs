use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

/// Configuration structure for a project.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Name of the project.
    pub project_name: String,
    /// Path to the project root directory.
    pub project_path: PathBuf,
    /// Path to the output file where the documentation will be saved.
    pub output_file: PathBuf,
    /// List of specific files to include in the documentation.
    pub files: Vec<PathBuf>,
    /// List of directories to include in the documentation (files within these directories will be processed recursively).
    pub directories: Vec<PathBuf>,
}

impl Config {
    /// Loads a project configuration from a YAML file.
    ///
    /// # Arguments
    ///
    /// * `config_path` - Path to the YAML configuration file.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Box<dyn std::error::Error>>` - The loaded configuration or an error.
    pub fn load(config_path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let config_content = fs::read_to_string(config_path)?;
        let config: Config = serde_yaml::from_str(&config_content)?;
        Ok(config)
    }
}