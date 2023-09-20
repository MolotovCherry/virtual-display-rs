use std::collections::BTreeMap;

use egui::{vec2, Align, Context, Grid, Id};

use crate::monitor::{MonitorMode, MonitorState, RefreshRate};

pub struct AddResolutionWindow;

impl AddResolutionWindow {
    pub fn new() -> Self {
        Self
    }

    pub fn show(self, ctx: &Context, state: &mut MonitorState) {
        egui::Window::new("Add Resolution")
            .id(Id::new(format!("add_res{}", state.monitor.id)))
            .open(&mut state.add_resolution_window)
            .fixed_size([60.0, f32::INFINITY])
            .show(ctx, |ui| {
                Grid::new(ui.id().with(state.monitor.id))
                    .num_columns(2)
                    .striped(true)
                    .min_col_width(60.0)
                    .spacing(vec2(ui.available_width(), 20.0))
                    .show(ui, |ui| {
                        let mut w = if state.tmp_w > 0 {
                            state.tmp_w.to_string()
                        } else {
                            "".to_string()
                        };
                        let mut h = if state.tmp_h > 0 {
                            state.tmp_h.to_string()
                        } else {
                            "".to_string()
                        };
                        let mut r = if state.tmp_r > 0 {
                            state.tmp_r.to_string()
                        } else {
                            "".to_string()
                        };

                        let w_widget =
                            egui::TextEdit::singleline(&mut w).vertical_align(Align::Center);
                        let h_widget =
                            egui::TextEdit::singleline(&mut h).vertical_align(Align::Center);
                        let r_widget = egui::TextEdit::singleline(&mut r)
                            .vertical_align(Align::Center)
                            .hint_text("60");

                        ui.label("Width");
                        let res_w = ui.add_sized(vec2(50.0, 20.0), w_widget);
                        ui.end_row();
                        ui.label("Height");
                        let res_h = ui.add_sized(vec2(50.0, 20.0), h_widget);
                        ui.end_row();
                        ui.label("Refresh Rate");
                        let res_r = ui.add_sized(vec2(50.0, 20.0), r_widget);
                        ui.end_row();

                        if res_w.changed() {
                            if let Ok(w) = w.parse::<u32>() {
                                state.tmp_w = w;
                            } else if w.is_empty() {
                                state.tmp_w = 0;
                            }
                        }

                        if res_h.changed() {
                            if let Ok(h) = h.parse::<u32>() {
                                state.tmp_h = h;
                            } else if h.is_empty() {
                                state.tmp_h = 0;
                            }
                        }

                        if res_r.changed() {
                            if let Ok(r) = r.parse::<u32>() {
                                state.tmp_r = r;
                            } else if r.is_empty() {
                                state.tmp_r = 0;
                            }
                        }

                        let add = ui.button("+").on_hover_text("Add resolution");

                        // Add the resolution + refresh rate to pending changes
                        // if monitor resolution already exists, this does nothing
                        #[allow(clippy::all)]
                        if add.clicked() {
                            if state.tmp_h > 0 && state.tmp_w > 0 && state.tmp_r > 0 {
                                // resolution already exists, user should edit the already existing one instead
                                let exists = state.monitor.modes.as_ref().is_some_and(|i| {
                                    i.iter().any(|(_, mode)| {
                                        mode.width == state.tmp_w && mode.height == state.tmp_h
                                    })
                                });

                                if !exists {
                                    if let Some(modes) = state.monitor.modes.as_mut() {
                                        modes.insert(
                                            format!("{}x{}", state.tmp_w, state.tmp_h),
                                            MonitorMode {
                                                width: state.tmp_w,
                                                height: state.tmp_h,
                                                refresh_rates: vec![RefreshRate {
                                                    rate: state.tmp_r,
                                                    pending: true,
                                                }],
                                                pending: true,
                                            },
                                        );
                                    } else {
                                        let mut map = BTreeMap::new();
                                        map.insert(
                                            format!("{}x{}", state.tmp_w, state.tmp_h),
                                            MonitorMode {
                                                width: state.tmp_w,
                                                height: state.tmp_h,
                                                refresh_rates: vec![RefreshRate {
                                                    rate: state.tmp_r,
                                                    pending: true,
                                                }],
                                                pending: true,
                                            },
                                        );
                                        state.monitor.modes = Some(map);
                                    }

                                    state.tmp_w = 0;
                                    state.tmp_h = 0;
                                    state.tmp_r = 0;
                                }
                            }
                        }
                    });
            });
    }
}
