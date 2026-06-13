use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use anyhow::{Context, Result};

/// Loads language definitions from a YAML file.
///
/// # Arguments
///
/// * `languages_path` - Path to the YAML file containing language definitions.
///
/// # Returns
///
/// * `Result<HashMap<String, String>>` - A map of file extensions to language names.
pub fn load_languages(languages_path: &std::path::Path) -> Result<HashMap<String, String>> {
    let content = fs::read_to_string(languages_path)
        .context(format!("Failed to read languages file: {}", languages_path.display()))?;
    
    let config: LanguageConfig = serde_yaml::from_str(&content)
        .context(format!("Failed to parse languages file: {}", languages_path.display()))?;
    
    Ok(config.languages)
}

#[derive(Deserialize)]
struct LanguageConfig {
    languages: HashMap<String, String>,
}