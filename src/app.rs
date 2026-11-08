//! FlowRail's original signal-track UI: a vertical timeline, not a floating pill.
use std::time::{Duration, Instant};

use eframe::egui::{self, Color32, RichText};

use crate::{
    core::{
        activity::{Activity, ActivityFeed, Priority},
        focus::{FocusPhase, FocusTimer},
    },
    platform::{self, QuickAction},
    settings::Settings,
    theme,
};

pub struct FlowRailApp {
    focus: FocusTimer,
    settings: Settings,
    started_at: Instant,
    message: Option<String>,
}

impl FlowRailApp {
    pub fn new() -> Self {
        let (settings, message) = match Settings::load() {
            Ok(settings) => (settings, None),
            Err(error) => (Settings::default(), Some(error)),
        };
        Self {
            focus: FocusTimer::new(Duration::from_secs(25 * 60)),
            settings,
            started_at: Instant::now(),
            message,
        }
    }

    fn save_settings(&mut self) {
        self.message = self.settings.save().err();
    }

    fn launch(&mut self, action: QuickAction) {
        if let Err(error) = platform::launch(action) {
            self.message = Some(format!("Could not open the requested app: {error}"));
        }
    }

    fn header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("F L O W R A I L").size(19.0).strong().color(theme::TEXT));
                ui.label(RichText::new("A SIGNAL TRACK FOR WINDOWS").size(10.0).color(theme::MUTED));
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Exit  ×").clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            ui.colored_label(theme::ACCENT, "●");
            ui.label(RichText::new("LOCAL SESSION").size(11.0).color(theme::ACCENT).strong());
            ui.separator();
            ui.label(RichText::new("NO ACCOUNT  ·  NO CLOUD").size(10.0).color(theme::MUTED));
        });
    }

    fn focus_card(&mut self, ui: &mut egui::Ui, now: Instant) {
        egui::Frame::new()
            .fill(theme::SURFACE)
            .stroke(egui::Stroke::new(1.0, theme::BORDER))
            .corner_radius(18)
            .inner_margin(16)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                section_label(ui, "01 / FOCUS LANE", theme::ACCENT);
                ui.add_space(4.0);
                let remaining = self.focus.remaining(now).as_secs();
                ui.label(
                    RichText::new(format!("{:02}:{:02}", remaining / 60, remaining % 60))
                        .size(40.0)
                        .monospace()
                        .strong()
                        .color(theme::TEXT),
                );
                let status = match self.focus.phase() {
                    FocusPhase::Idle => "Start a 25-minute focus session",
                    FocusPhase::Running => "In progress · interruptions stay out of your way",
                    FocusPhase::Paused => "Paused · resume when ready",
                    FocusPhase::Complete => "Session complete · good work",
                };
                ui.label(RichText::new(status).size(12.0).color(theme::MUTED));
                ui.add_space(8.0);
                ui.add(
                    egui::ProgressBar::new(self.focus.progress(now))
                        .fill(theme::ACCENT)
                        .desired_width(ui.available_width()),
                );
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    match self.focus.phase() {
                        FocusPhase::Idle | FocusPhase::Complete => {
                            if ui.button("▶  Start session").clicked() {
                                self.focus.start(now);
                            }
                        }
                        FocusPhase::Running => {
                            if ui.button("Ⅱ  Pause").clicked() {
                                self.focus.pause(now);
                            }
                        }
                        FocusPhase::Paused => {
                            if ui.button("▶  Resume").clicked() {
                                self.focus.resume(now);
                            }
                        }
                    }
                    if self.focus.phase() != FocusPhase::Idle && ui.button("Reset").clicked() {
                        self.focus.reset();
                    }
                });
            });
    }

    fn activity_card(&self, ui: &mut egui::Ui, now: Instant) {
        let mut feed = ActivityFeed::default();
        if self.focus.phase() != FocusPhase::Idle {
            let (title, priority) = match self.focus.phase() {
                FocusPhase::Running => ("Focus session active", Priority::Active),
                FocusPhase::Paused => ("Focus session paused", Priority::Background),
                FocusPhase::Complete => ("Focus session complete", Priority::Urgent),
                FocusPhase::Idle => unreachable!(),
            };
            feed.upsert(Activity {
                id: "focus",
                title: title.into(),
                detail: format!("{} minutes remaining", self.focus.remaining(now).as_secs() / 60),
                priority,
            });
        }
        if let Some(uptime) = platform::system_uptime() {
            feed.upsert(Activity {
                id: "windows-uptime",
                title: "Windows running".into(),
                detail: format!("System uptime: {}h {}m", uptime.as_secs() / 3600, (uptime.as_secs() / 60) % 60),
                priority: Priority::Background,
            });
        }
        egui::Frame::new()
            .fill(theme::SURFACE)
            .stroke(egui::Stroke::new(1.0, theme::BORDER))
            .corner_radius(18)
            .inner_margin(16)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                section_label(ui, "02 / ACTIVITY STREAM", theme::ACCENT);
                ui.add_space(4.0);
                let visible = feed.visible(self.settings.quiet_mode);
                if visible.is_empty() {
                    ui.label(RichText::new("All clear. Nothing needs your attention.").color(theme::MUTED));
                }
                for entry in visible {
                    ui.horizontal(|ui| {
                        let color = if entry.priority == Priority::Urgent { theme::WARNING } else { theme::ACCENT };
                        ui.colored_label(color, "▌");
                        ui.vertical(|ui| {
                            ui.label(RichText::new(&entry.title).color(theme::TEXT).strong());
                            if !self.settings.compact_mode {
                                ui.label(RichText::new(&entry.detail).size(11.0).color(theme::MUTED));
                            }
                        });
                    });
                    ui.add_space(6.0);
                }
            });
    }

    fn actions_card(&mut self, ui: &mut egui::Ui) {
        egui::Frame::new()
            .fill(theme::SURFACE_RAISED)
            .corner_radius(18)
            .inner_margin(16)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                section_label(ui, "03 / QUICK ACTIONS", theme::ACCENT);
                ui.add_space(5.0);
                ui.horizontal_wrapped(|ui| {
                    if ui.button("▣  Explorer").clicked() {
                        self.launch(QuickAction::Explorer);
                    }
                    if ui.button("⚙  Settings").clicked() {
                        self.launch(QuickAction::Settings);
                    }
                    if ui.button("▤  Task Manager").clicked() {
                        self.launch(QuickAction::TaskManager);
                    }
                });
            });
    }

    fn footer(&mut self, ui: &mut egui::Ui, now: Instant) {
        ui.horizontal(|ui| {
            let quiet_changed = ui.checkbox(&mut self.settings.quiet_mode, "Quiet").changed();
            let compact_changed = ui.checkbox(&mut self.settings.compact_mode, "Compact").changed();
            if quiet_changed || compact_changed {
                self.save_settings();
            }
        });
        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("App uptime {}m", now.duration_since(self.started_at).as_secs() / 60))
                .size(10.0).color(theme::MUTED));
            ui.label(RichText::new("·  prototype v0.1").size(10.0).color(theme::MUTED));
        });
        if let Some(message) = &self.message {
            ui.label(RichText::new(message).color(theme::WARNING).size(11.0));
        }
    }
}

impl eframe::App for FlowRailApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        self.focus.tick(now);

        // The timer only displays whole seconds. Repaint once per second while
        // it runs, and at most once per minute while idle. Input and window
        // events trigger immediate frames through egui; no busy render loop.
        let refresh_after = if self.focus.phase() == FocusPhase::Running {
            Duration::from_secs(1)
        } else {
            Duration::from_secs(60)
        };
        ui.ctx().request_repaint_after(refresh_after);

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(theme::BACKGROUND).inner_margin(18))
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = egui::vec2(9.0, 9.0);
                self.header(ui);
                ui.add_space(12.0);
                self.focus_card(ui, now);
                ui.add_space(7.0);
                self.activity_card(ui, now);
                ui.add_space(7.0);
                self.actions_card(ui);
                ui.add_space(13.0);
                self.footer(ui, now);
            });
    }
}

fn section_label(ui: &mut egui::Ui, label: &str, color: Color32) {
    ui.label(RichText::new(label).size(11.0).strong().color(color));
}
