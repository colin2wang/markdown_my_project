use std::path::{Path, PathBuf};
use std::collections::BTreeMap;
use anyhow::Result;

/// Represents a directory in the project tree.
#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<String>,
    subdirectories: BTreeMap<String, Directory>,
}

impl Directory {
    fn new(name: String) -> Self {
        Directory {
            name,
            files: Vec::new(),
            subdirectories: BTreeMap::new(),
        }
    }

    fn add_file(&mut self, file: String) {
        self.files.push(file);
    }
}

/// Builds a directory tree from a list of files.
///
/// # Arguments
///
/// * `files` - List of files with their paths and contents.
/// * `project_root` - Path to the project root directory.
///
/// # Returns
///
/// * `Directory` - The root directory of the tree.
fn build_directory_tree(files: &[(PathBuf, String)], project_root: &Path) -> Directory {
    let mut root = Directory::new("".to_string());

    for (file_path, _) in files {
        let relative_path = file_path.strip_prefix(project_root).unwrap_or(file_path);
        let components: Vec<String> = relative_path
            .components()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .collect();

        let mut current_dir = &mut root;
        for (i, component) in components.iter().enumerate() {
            if i < components.len() - 1 {
                let entry = current_dir.subdirectories.entry(component.clone());
                let sub_dir = entry.or_insert_with(|| Directory::new(component.clone()));
                current_dir = sub_dir;
            } else {
                current_dir.add_file(component.clone());
            }
        }
    }

    root
}

/// Converts a directory tree to a string representation.
///
/// # Arguments
///
/// * `directory` - The directory to convert.
/// * `indent` - Current indentation string.
/// * `is_last` - Whether this is the last item in its parent.
/// * `is_root` - Whether this is the root directory.
///
/// # Returns
///
/// * `String` - The string representation of the directory tree.
fn directory_tree_to_string(
    directory: &Directory,
    indent: &str,
    is_last: bool,
    is_root: bool,
) -> String {
    let mut tree = String::new();
    if !is_root {
        let connector = if is_last { "└── " } else { "├── " };
        let line = format!("{}{}{}/", indent, connector, directory.name);
        tree.push_str(&line);
        tree.push('\n');
    }

    // Calculate new indent based on whether current directory is last or root
    let new_indent = if is_root {
        indent.to_string()
    } else if is_last {
        format!("{}{}", indent, "    ")
    } else {
        format!("{}{}", indent, "│   ")
    };

    let total_items = directory.files.len() + directory.subdirectories.len();
    let mut current_item = 0;

    // Process files
    for file in &directory.files {
        current_item += 1;
        let is_last_item = current_item == total_items;
        let connector = if is_last_item { "└── " } else { "├── " };
        tree.push_str(&format!("{}{}{}\n", new_indent, connector, file));
    }

    // Process subdirectories
    for sub_dir in directory.subdirectories.values() {
        current_item += 1;
        let is_last_item = current_item == total_items;
        let sub_tree = directory_tree_to_string(sub_dir, &new_indent, is_last_item, false);
        tree.push_str(&sub_tree);
    }

    tree
}

/// Generates a tree-like structure of the project files.
///
/// # Arguments
///
/// * `project_name` - Name of the project.
/// * `files` - List of files with their paths.
/// * `project_root` - Path to the project root directory.
///
/// # Returns
///
/// * `Result<String>` - The tree structure as a string.
pub fn generate_tree(
    project_name: &str,
    files: &[(PathBuf, String)],
    project_root: &Path,
) -> Result<String> {
    let mut root = build_directory_tree(files, project_root);
    root.name = project_name.to_string();

    let tree = directory_tree_to_string(&root, "", true, true);
    Ok(tree)
}
