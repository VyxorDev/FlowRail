#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod core;
mod platform;
mod settings;
mod theme;

use eframe::egui;

fn main() -> eframe::Result {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = egui::ViewportBuilder::default()
        .with_title("FlowRail")
        .with_inner_size([410.0, 690.0])
        .with_resizable(false)
        .with_decorations(false)
        .with_movable_by_background(true);

    // This is a lightweight 2D companion, not a game renderer. Prefer the
    // integrated/lowest-power adapter so a hybrid laptop is not encouraged
    // to wake its discrete GPU just to draw the rail.
    if let eframe::egui_wgpu::WgpuSetup::CreateNew(mut setup) =
        native_options.wgpu_options.wgpu_setup.clone()
    {
        setup.power_preference = eframe::egui_wgpu::wgpu::PowerPreference::LowPower;
        native_options.wgpu_options.wgpu_setup =
            eframe::egui_wgpu::WgpuSetup::CreateNew(setup);
    }

    eframe::run_native(
        "FlowRail",
        native_options,
        Box::new(|cc| {
            theme::apply(&cc.egui_ctx);
            Ok(Box::new(app::FlowRailApp::new()))
        }),
    )
}
