# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added
- `--projects-dir`, `--languages-file`, `--output-dir`, `--verbose` command line arguments
- File exclusion patterns with glob support (`exclude_patterns`)
- File size limit configuration (`max_file_size`)
- Parallel file processing using `rayon`
- Progress bar display during documentation generation
- Unit tests for file processor
- `build.bat` script for automated release builds

### Changed
- Refactored error handling to use `anyhow` crate
- Moved tree generation logic to separate `tree_generator` module
- Moved `## Project File Tree` section to the top of generated documentation
- Improved directory exclusion logic to support name-based matching
- Updated dependencies: `clap`, `anyhow`, `walkdir`, `glob`, `indicatif`, `rayon`

### Fixed
- Fixed tree structure display issues (incorrect indentation and connectors)
- Fixed UTF-8 encoding for output files

## [0.1.0] - 2024-01-01

### Added
- Initial release
- YAML configuration support
- Recursive file processing
- Language detection based on file extensions
- Markdown generation with syntax highlighting
- Logging with log4rs
- Multi-project support

### Changed
- Replaced simplelog with log4rs for better logging
- Added rolling file appender for log rotation

### Fixed
- Fixed project file tree generation issues
- Added `/` after folder names in tree output

---

For more details, see the [commit history](https://github.com/colin2wang/markdown_my_project/commits/main).
