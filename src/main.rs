mod config;
mod file_processor;
mod markdown_generator;
mod logger;
mod language;
mod tree_generator;

use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the projects directory
    #[arg(short, long, default_value = "projects")]
    projects_dir: PathBuf,

    /// Path to the languages definition file
    #[arg(short, long, default_value = "languages.yml")]
    languages_file: PathBuf,

    /// Output directory for generated documentation
    #[arg(short, long, default_value = "output")]
    output_dir: PathBuf,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize the logger with the configuration from log4rs.yml
    logger::init_logger().context("Failed to initialize logger")?;

    log::info!("Starting to generate project documentation...");

    // Load common language definitions from the YAML file
    let languages = language::load_languages(&args.languages_file)
        .context("Failed to load language definitions")?;
    log::info!("Loaded language definitions from {}", args.languages_file.display());

    // Ensure output directory exists
    fs::create_dir_all(&args.output_dir)
        .context("Failed to create output directory")?;

    // Get list of project configuration files
    let config_files = get_config_files(&args.projects_dir)?;
    let total_projects = config_files.len();

    if total_projects == 0 {
        log::warn!("No project configuration files found in {}", args.projects_dir.display());
        return Ok(());
    }

    // Create progress bar
    let pb = ProgressBar::new(total_projects as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    // Process each project configuration
    for config_path in config_files {
        let config_name = config_path.file_stem()
            .unwrap_or_default()
            .to_string_lossy();
        
        pb.set_message(format!("Processing: {}", config_name));

        // Load the project configuration from the YAML file
        let config = config::Config::load(&config_path)
            .context(format!("Failed to load configuration: {}", config_path.display()))?;
        log::info!("Loaded project configuration: {}", config.project_name);

        // Get the project root directory
        let project_root = Path::new(&config.project_path);

        // Process files and directories specified in the configuration
        let files = file_processor::process_files(
            &config.project_path,
            &config.files,
            &config.directories,
            &config.exclude_directories,
            &config.exclude_patterns,
            config.max_file_size,
        ).context(format!("Failed to process files for project: {}", config.project_name))?;
        log::info!("Processed {} files for project: {}", files.len(), config.project_name);

        // Generate Markdown content for the project documentation
        let markdown_content = markdown_generator::generate_markdown(
            &config.project_name,
            files,
            &languages,
            project_root,
        ).context(format!("Failed to generate markdown for project: {}", config.project_name))?;

        // Write the generated Markdown content to the output file with UTF-8 encoding
        let output_path = args.output_dir.join(&config.output_file);
        use std::io::Write;
        let mut file = fs::File::create(&output_path)
            .context(format!("Failed to create output file: {}", output_path.display()))?;
        file.write_all(markdown_content.as_bytes())
            .context(format!("Failed to write output file: {}", output_path.display()))?;
        log::info!("Generated documentation for project: {} -> {}", config.project_name, output_path.display());

        pb.inc(1);
    }

    pb.finish_with_message("Done");
    log::info!("Project documentation generation complete.");
    Ok(())
}

/// Get all YAML configuration files from the projects directory
fn get_config_files(projects_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut config_files = Vec::new();
    
    if !projects_dir.exists() {
        return Ok(config_files);
    }

    for entry in fs::read_dir(projects_dir)
        .context(format!("Failed to read projects directory: {}", projects_dir.display()))? {
        let entry = entry.context("Failed to read directory entry")?;
        let config_path = entry.path();
        
        if config_path.is_file() && config_path.extension().unwrap_or_default() == "yml" {
            config_files.push(config_path);
        }
    }

    // Sort for consistent ordering
    config_files.sort();
    Ok(config_files)
}
