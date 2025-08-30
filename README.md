
# Disk Scout

A visual disk space analyzer, built in Rust with `egui`. This tool scans a specified directory and displays its contents as a treemap, allowing you to quickly identify which files and folders are consuming the most space.


## Features

  * **Recursive Directory Scanning:** Efficiently traverses the file system to build a complete map of a directory's contents.
  * **Treemap Visualization:** Displays the file system hierarchy as a set of nested rectangles, where the area of each rectangle is proportional to the size of the file or folder it represents.
  * **Interactive Tooltips:** Hover over any rectangle to see the name and size of the corresponding file or folder.
  * **Dynamic Resizing:** The treemap layout automatically adjusts to the window size, providing a responsive user experience.
  * **UI Controls:** Allows you to specify a directory to scan at runtime using a text input field and a "Scan" button.
  * **Cross-Platform:** Built with Rust and `egui`, this application can be compiled and run on macOS, Windows, and Linux.

## Getting Started

### Prerequisites

You must have the Rust toolchain installed on your system. If you don't, you can install it from [rust-lang.org](https://www.rust-lang.org/tools/install).

### Installation & Usage

1.  **Clone the repository:**

    ```sh
    git clone https://github.com/k3rnel-paN1c5/disk-scout.git
    cd disk-scout
    ```

2.  **Build and run the application:**

    ```sh
    cargo run --release
    ```

    The application will start and automatically scan the directory it was run from.

## Project Structure

The project's logic is separated into three main files within the `src/` directory:

  * **`main.rs`**: Handles the application's entry point, GUI state management, and rendering loop using `eframe`/`egui`. It is responsible for drawing the treemap and handling user interaction.
  * **`scanner.rs`**: Contains the logic for traversing the file system. Its `build_tree` function recursively scans a path and constructs a `FileSystemNode` tree, which serves as the data model for the application.
  * **`treemap.rs`**: Implements the treemap layout algorithm. The `generate_treemap` function takes the `FileSystemNode` tree and a set of bounds (the window size) and returns a flat list of `TreemapNode`s, each with calculated coordinates and dimensions ready for drawing.

## Future Enhancements

This project is a solid foundation. Future improvements could include:

  * **Color Coding:** Assigning colors to rectangles based on file type or depth in the directory tree to make the visualization more informative.
  * **Click to Zoom:** Implementing functionality to click on a directory's rectangle to "zoom in" and view its contents as a new treemap.
  * **Background Scanning:** Moving the file system scan to a separate thread to prevent the UI from freezing while scanning large directories.
