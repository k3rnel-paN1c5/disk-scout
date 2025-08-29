//! This module provides the logic for generating a treemap layout.
//! It takes a hierarchical `FileSystemNode` tree and calculates the rectangular
//! coordinates and dimensions needed to visualize it.

use crate::scanner::FileSystemNode;

/// Represents a 2D rectangle with floating-point coordinates and dimensions.
/// This is used to define the boundaries for each node in the treemap.
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// Represents a single, drawable item in our treemap layout.
/// It links the file system data (name, size) to a specific `Rectangle`.
#[derive(Debug)]
pub struct TreemapNode {
    pub rect: Rectangle,
    pub name: String, 
    pub size: u64,
}

/// Generates a treemap layout from a `FileSystemNode` tree.
///
/// # Arguments
///
/// * `node` - A reference to the root `FileSystemNode` of the tree.
/// * `bounds` - The initial rectangle (e.g., the window) to fit the treemap into.
///
/// # Returns
///
/// A flat vector of `TreemapNode`'s, each representing a rectangle to be drawn.
pub fn generate_treemap(node: &FileSystemNode, bounds: Rectangle) -> Vec<TreemapNode> {
    let mut results = Vec::new();
    // The recursive helper function does the main work.
    calculate_layout(&node.children, bounds, &mut results, true);
    results
}

/// A recursive helper function that implements the "slice-and-dice" treemap algorithm.
///
/// It sorts children by size, then alternates between slicing the `bounds` rectangle
/// vertically and horizontally to position the children.
fn calculate_layout(
    nodes: &[FileSystemNode],
    bounds: Rectangle,
    results: &mut Vec<TreemapNode>,
    slice_vertically: bool,
) {
    if nodes.is_empty() {
        return;
    }

    let mut sorted_nodes = nodes.to_vec();
    sorted_nodes.sort_by(|a, b| b.size.cmp(&a.size));

    // Calculate the total size of all nodes at this level.
    let total_size = nodes.iter().map(|n| n.size).sum::<u64>() as f64;
    if total_size == 0.0 {
        return;
    }

     // Keep track of our position as we lay out rectangles.
    let mut current_x = bounds.x;
    let mut current_y = bounds.y;

    for node in nodes {
        // The proportion of the total size this node occupies.
        let proportion = node.size as f64 / total_size;
        let child_bounds;

         // Decide whether to slice horizontally or vertically.
        if slice_vertically {
            // Slice vertically: lay out children from left to right.
            let width = bounds.width * proportion;
            child_bounds = Rectangle {
                x: current_x,
                y: current_y,
                width,
                height: bounds.height,
            };
            current_x += width;
        } else {
            // Slice horizontally: lay out children from top to bottom.
            let height = bounds.height * proportion;
            child_bounds = Rectangle {
                x: current_x,
                y: current_y,
                width: bounds.width,
                height,
            };
            current_y += height;
        }
        
        results.push(TreemapNode {
            rect: child_bounds,
            name: node.name.clone(),
            size: node.size,
        });

        // Recursively call for the children, flipping the slice direction.
        if !node.children.is_empty() {
            calculate_layout(&node.children, child_bounds, results, !slice_vertically);
        }
    }
}