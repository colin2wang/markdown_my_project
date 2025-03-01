use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

/// Loads language definitions from a YAML file.
///
/// # Arguments
///
/// * `languages_path` - Path to the YAML file containing language definitions.
///
/// # Returns
///
/// * `Result<HashMap<String, String>, Box<dyn std::error::Error>>` - A map of file extensions to language names.
pub fn load_languages(languages_path: &std::path::Path) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(languages_path)?;
    let config: LanguageConfig = serde_yaml::from_str(&content)?;
    Ok(config.languages)
}

#[derive(Deserialize)]
struct LanguageConfig {
    languages: HashMap<String, String>,
}