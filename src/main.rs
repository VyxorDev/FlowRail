#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod core;
mod platform;
mod settings;
mod theme;

use eframe::egui;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("FlowRail")
            .with_inner_size([410.0, 690.0])
            .with_resizable(false)
            .with_decorations(false)
            .with_movable_by_background(true),
        ..Default::default()
    };

    eframe::run_native(
        "FlowRail",
        native_options,
        Box::new(|_cc| Ok(Box::new(app::FlowRailApp::new()))),
    )
}
