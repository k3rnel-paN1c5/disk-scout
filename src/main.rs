//! This is the main entry point and GUI logic for the Disk Scanner application.
//! It uses the `eframe` and `egui` libraries to create a native window and
//! render the treemap visualization.

mod scanner;
mod treemap;

use eframe::egui;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use treemap::{TreemapNode, Rectangle};
use scanner::FileSystemNode;

/// The main application struct that holds the state of the GUI.
struct DiskScannerApp {
    /// The result of the last scan. It's an Option containing a Result.
    /// - `None`: The initial state before a scan is run or when a scan is in progress.
    /// - `Some(Ok(tree))`: The scan was successful.
    /// - `Some(Err(e))`: The scan failed.
    scan_result: Option<Result<FileSystemNode, std::io::Error>>,
    /// A receiver for the result of the background scanning thread.
    scan_receiver: Option<Receiver<Result<FileSystemNode, std::io::Error>>>,
    /// The calculated layout of rectangles to be drawn. This is generated from a successful scan.
    layout: Option<Vec<TreemapNode>>,
    /// The size of the last frame, used to detect window resizing.
    last_frame_size: egui::Vec2,
}

impl Default for DiskScannerApp {
    fn default() -> Self {
        let (sender, receiver) = mpsc::channel();
        let path_to_scan = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        println!("Starting initial scan of: {}", path_to_scan.display());

        thread::spawn(move || {
            let result = scanner::build_tree(&path_to_scan);
            sender.send(result).expect("Failed to send scan result");
        });
        
        Self {
            scan_result: None, // Initially, we have no result.
            scan_receiver: Some(receiver), // We have a receiver to listen for the result.
            layout: None,
            last_frame_size: egui::Vec2::ZERO,
        }
    }
}

impl eframe::App for DiskScannerApp {
    /// This method is called once per frame and is responsible for all UI logic.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check if there's a result from the scanning thread.
        if let Some(receiver) = &self.scan_receiver {
            if let Ok(result) = receiver.try_recv() {
                self.scan_result = Some(result);
                self.scan_receiver = None; // We've received the result, so we can drop the receiver.
            }
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            // Check if the window size has changed. If so, recalculate the layout.
            let current_frame_size = ui.available_size();
            if self.last_frame_size != current_frame_size {
                if let Some(Ok(tree)) = &self.scan_result {
                    println!("Window resized, recalculating layout...");
                    let bounds = Rectangle { x: 0.0, y: 0.0, width: current_frame_size.x as f64, height: current_frame_size.y as f64 };
                    self.layout = Some(treemap::generate_treemap(tree, bounds));
                }
                self.last_frame_size = current_frame_size;
            }

            // --- UI LOGIC: Based on Scan State ---
            if self.scan_receiver.is_some() {
                 ui.centered_and_justified(|ui| {
                    ui.label("Scanning...");
                });
                return;
            }
            
            // Show an error message if the scan failed.
            if let Some(Err(e)) = &self.scan_result {
                ui.centered_and_justified(|ui| {
                    ui.label(format!("Error scanning directory:\n{}", e));
                });
                return;
            }

            // If the layout has been calculated, draw it.
            if let Some(layout) = &self.layout {
                let painter = ui.painter();
                let mut hovered_node: Option<&TreemapNode> = None;

                for node in layout {
                    let rect = egui::Rect::from_min_max(
                        egui::pos2(node.rect.x as f32, node.rect.y as f32),
                        egui::pos2((node.rect.x + node.rect.width) as f32, (node.rect.y + node.rect.height) as f32),
                    );
                    
                    // Don't draw rectangles that are too small to see.
                    if rect.width() < 1.0 || rect.height() < 1.0 { continue; }

                    painter.rect_filled(rect, 3.0, egui::Color32::from_gray(50));
                    painter.rect_stroke(rect, 3.0, egui::Stroke::new(1.0, egui::Color32::from_gray(150)));

                    // Check for hover to show a tooltip.
                    if ui.rect_contains_pointer(rect) {
                        hovered_node = Some(node);
                    }
                }

                if let Some(node) = hovered_node {
                    let tooltip_id = egui::Id::new("treemap_tooltip");
                    let tooltip_layer_id = egui::LayerId::new(egui::Order::Tooltip, tooltip_id);
                    egui::show_tooltip_at_pointer(ctx, tooltip_layer_id, tooltip_id, |ui| {
                        ui.label(format!("Name: {}", node.name));
                        ui.label(format!("Size: {} bytes", node.size));
                    });
                }
            } else if self.scan_result.is_some() {
                // This is shown if the scan was successful but the layout hasn't been generated yet.
                 ui.centered_and_justified(|ui| {
                    ui.label("Generating layout...");
                });
            }
        });

        // Trigger a repaint. This is important for the resizing logic to work smoothly.
        ctx.request_repaint();
    }
}


/// The main entry point of the application.
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Disk Scanner",
        options,
        Box::new(|_cc| Ok(Box::new(DiskScannerApp::default()))),
    )
}