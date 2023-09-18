use egui::{vec2, Align, Context, Id};

use crate::app::MonitorState;

pub struct MonitorWindow;

impl MonitorWindow {
    pub fn new() -> Self {
        Self
    }

    pub fn show(&self, ctx: &Context, monitor: &mut MonitorState) {
        let name = if !monitor.name.is_empty() {
            monitor.name.clone()
        } else {
            format!("Monitor {}", monitor.id)
        };

        egui::Window::new(name)
            .id(Id::new(format!("mon{}", monitor.id)))
            .auto_sized()
            .open(&mut monitor.monitor_window)
            .show(ctx, |ui| {
                egui::Grid::new(Id::new(monitor.id))
                    .striped(true)
                    .spacing(vec2(80.0, 20.0))
                    .show(ui, |ui| {
                        ui.label("Name");

                        let name = egui::TextEdit::singleline(&mut monitor.name)
                            .vertical_align(Align::Center);

                        ui.add_sized(vec2(100.0, ui.available_height()), name);

                        ui.end_row();
                    });
            });
    }
}
