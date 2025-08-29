//! This module is responsible for scanning the file system.
//! It contains the logic to recursively traverse a directory and build a hierarchical
//! tree structure representing its contents.

use std::fs;
use std::path::Path;

/// Represents a node in the file system tree.
/// It can be either a file or a directory, and it owns its data.
#[derive(Debug, Clone)]
pub struct FileSystemNode {
    /// The name of the file or directory (e.g., "src", "main.rs").
    pub name: String,
    /// The total size of the node in bytes. For a file, it's the file size.
    /// For a directory, it's the sum of the sizes of all its children.
    pub size: u64,
    /// A vector of child nodes. This is empty for files.
    pub children: Vec<FileSystemNode>,
}

/// Recursively scans a directory and builds a tree of `FileSystemNode`'s.
///
/// This function walks through the file system starting from the given path.
/// It calculates the size of directories by summing their children's sizes.
///
/// # Arguments
///
/// * `path` - The path to the directory or file to build the tree from.
///
/// # Returns
///
/// A `Result` containing the root `FileSystemNode` of the scanned tree,
/// or an `io::Error` if scanning fails at the root level.
pub fn build_tree(path: &Path) -> Result<FileSystemNode, std::io::Error> {
    let metadata = fs::metadata(path)?;

    // Get the name of the file or directory from the path.
    let name = path
        .file_name()
        .unwrap_or_else(|| path.as_os_str()) // Fallback for paths like "/" or "."
        .to_string_lossy()
        .into_owned();

    if metadata.is_dir() {
        let mut children = Vec::new();
        let mut total_size = 0;

        // Read all entries in the directory.
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let child_path = entry.path();
            
            // Recursively call build_tree for each child.
            match build_tree(&child_path) {
                Ok(child_node) => {
                    total_size += child_node.size;
                    children.push(child_node);
                }
                Err(e) => {
                    // Log an error for inaccessible files/dirs but continue scanning others.
                    // This makes the scan more resilient to permission errors.
                    eprintln!("Failed to scan {}: {}", child_path.display(), e);
                }
            }
        }

        Ok(FileSystemNode {
            name,
            size: total_size,
            children,
        })
    } else {
        // It's a file, so it has a defined size and no children.
        Ok(FileSystemNode {
            name,
            size: metadata.len(),
            children: Vec::new(), 
        })
    }
}
