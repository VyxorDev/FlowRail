//! FlowRail's visual system: quiet graphite, warm text and a restrained mint signal.
use eframe::egui::{self, Color32, CornerRadius, Stroke};

pub const BACKGROUND: Color32 = Color32::from_rgb(11, 15, 22);
pub const SURFACE: Color32 = Color32::from_rgb(19, 25, 35);
pub const SURFACE_RAISED: Color32 = Color32::from_rgb(27, 36, 49);
pub const SURFACE_HOVER: Color32 = Color32::from_rgb(37, 49, 64);
pub const BORDER: Color32 = Color32::from_rgb(45, 57, 73);
pub const TEXT: Color32 = Color32::from_rgb(238, 241, 237);
pub const MUTED: Color32 = Color32::from_rgb(151, 165, 177);
pub const ACCENT: Color32 = Color32::from_rgb(116, 222, 190);
pub const WARNING: Color32 = Color32::from_rgb(239, 190, 122);

/// Apply the visual system once during startup, not on every frame.
pub fn apply(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = BACKGROUND;
    visuals.window_fill = SURFACE;
    visuals.window_stroke = Stroke::new(1.0, BORDER);
    visuals.window_corner_radius = CornerRadius::same(18);

    visuals.widgets.noninteractive.bg_fill = BACKGROUND;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, BORDER);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, TEXT);

    visuals.widgets.inactive.bg_fill = SURFACE_RAISED;
    visuals.widgets.inactive.weak_bg_fill = SURFACE_RAISED;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, BORDER);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, TEXT);
    visuals.widgets.inactive.corner_radius = CornerRadius::same(10);

    visuals.widgets.hovered.bg_fill = SURFACE_HOVER;
    visuals.widgets.hovered.weak_bg_fill = SURFACE_HOVER;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, ACCENT);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, TEXT);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(10);

    visuals.widgets.active.bg_fill = ACCENT;
    visuals.widgets.active.weak_bg_fill = ACCENT;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, ACCENT);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, BACKGROUND);
    visuals.widgets.active.corner_radius = CornerRadius::same(10);

    visuals.selection.bg_fill = Color32::from_rgb(39, 75, 67);
    visuals.selection.stroke = Stroke::new(1.0, ACCENT);
    visuals.hyperlink_color = ACCENT;
    ctx.set_visuals(visuals);

    ctx.style_mut(|style| {
        style.spacing.item_spacing = egui::vec2(10.0, 8.0);
        style.spacing.button_padding = egui::vec2(14.0, 9.0);
        style.spacing.interact_size = egui::vec2(44.0, 38.0);
        style.animation_time = 0.16;
    });
}
