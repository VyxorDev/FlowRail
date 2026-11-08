#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod core;
mod platform;
mod settings;
mod theme;

use eframe::egui;

fn main() -> eframe::Result {
    // Configure the renderer before constructing NativeOptions so the
    // options remain a single, clippy-clean struct initializer.
    let mut wgpu_options = eframe::egui_wgpu::WgpuConfiguration::default();

    // This is a lightweight 2D companion, not a game renderer. Prefer the
    // integrated/lowest-power adapter so a hybrid laptop is not encouraged
    // to wake its discrete GPU just to draw the rail.
    if let eframe::egui_wgpu::WgpuSetup::CreateNew(setup) = &mut wgpu_options.wgpu_setup {
        setup.power_preference = eframe::egui_wgpu::wgpu::PowerPreference::LowPower;
    }

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("FlowRail")
            .with_inner_size([410.0, 690.0])
            .with_resizable(false)
            .with_decorations(false)
            .with_movable_by_background(true),
        wgpu_options,
        ..Default::default()
    };

    eframe::run_native(
        "FlowRail",
        native_options,
        Box::new(|cc| {
            theme::apply(&cc.egui_ctx);
            Ok(Box::new(app::FlowRailApp::new()))
        }),
    )
}
