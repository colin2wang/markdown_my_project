use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result};

/// Configuration structure for a project.
///
/// This struct represents the configuration for a project, including its name,
/// root directory path, output file path, specific files to include, directories
/// to include recursively, and directories to exclude.
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
    /// List of directories to exclude from the documentation.
    #[serde(default)]
    pub exclude_directories: Vec<String>,
    /// List of glob patterns to exclude files (e.g., "*.log", "target/**").
    #[serde(default)]
    pub exclude_patterns: Vec<String>,
    /// Maximum file size in bytes. Files larger than this will be skipped.
    /// If not specified, no limit is applied.
    #[serde(default)]
    pub max_file_size: Option<u64>,

    /// Markdown output language. "zh_cn" for Chinese, "en_us" for English (default).
    #[serde(default = "default_markdown_lang")]
    pub markdown_lang: String,
}

fn default_markdown_lang() -> String {
    "en_us".to_string()
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
    /// * `Result<Self>` - The loaded configuration or an error.
    pub fn load(config_path: &PathBuf) -> Result<Self> {
        let config_content = fs::read_to_string(config_path)
            .context(format!("Failed to read configuration file: {}", config_path.display()))?;
        
        let config: Config = serde_yaml::from_str(&config_content)
            .context(format!("Failed to parse configuration file: {}", config_path.display()))?;
        
        // Validate configuration
        config.validate()
            .context(format!("Invalid configuration in: {}", config_path.display()))?;
        
        Ok(config)
    }

    /// Validates the configuration values.
    fn validate(&self) -> Result<Self> {
        // Validate project name
        if self.project_name.trim().is_empty() {
            anyhow::bail!("Project name cannot be empty");
        }

        // Validate project path
        if !self.project_path.exists() {
            anyhow::bail!("Project path does not exist: {}", self.project_path.display());
        }
        if !self.project_path.is_dir() {
            anyhow::bail!("Project path is not a directory: {}", self.project_path.display());
        }

        // Validate output file extension
        if let Some(ext) = self.output_file.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            if !["md", "markdown", "html", "txt"].contains(&ext_str.as_str()) {
                log::warn!("Output file extension '{}' might not be supported", ext_str);
            }
        }

        // Validate max file size
        if let Some(max_size) = self.max_file_size {
            if max_size == 0 {
                anyhow::bail!("Max file size cannot be zero");
            }
        }

        // Validate exclude patterns
        for pattern in &self.exclude_patterns {
            if pattern.trim().is_empty() {
                anyhow::bail!("Exclude pattern cannot be empty");
            }
        }

        Ok(self.clone())
    }
}

impl Clone for Config {
    fn clone(&self) -> Self {
        Config {
            project_name: self.project_name.clone(),
            project_path: self.project_path.clone(),
            output_file: self.output_file.clone(),
            files: self.files.clone(),
            directories: self.directories.clone(),
            exclude_directories: self.exclude_directories.clone(),
            exclude_patterns: self.exclude_patterns.clone(),
            max_file_size: self.max_file_size,
            markdown_lang: self.markdown_lang.clone(),
        }
    }
}
