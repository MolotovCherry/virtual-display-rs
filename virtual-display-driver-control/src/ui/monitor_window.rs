use std::cell::RefCell;

use driver_ipc::DriverCommand;
use egui::{vec2, Align, CollapsingHeader, Color32, Context, Grid, Id, Layout};

use crate::{
    app::TcpWrapper,
    ipc::ipc_call,
    monitor::{MonitorState, RefreshRate},
};

use super::add_resolution_window::AddResolutionWindow;

pub struct MonitorWindow<'a> {
    con: &'a RefCell<TcpWrapper>,
}

impl<'a> MonitorWindow<'a> {
    pub fn new(con: &'a RefCell<TcpWrapper>) -> Self {
        Self { con }
    }

    pub fn show(&self, ctx: &Context, state: &mut MonitorState) {
        let name = if !state.name.is_empty() {
            state.name.clone()
        } else {
            format!("Monitor {}", state.monitor.id)
        };

        egui::Window::new(name)
            .id(Id::new(format!("mon{}", state.monitor.id)))
            .open(&mut state.monitor_window)
            .resizable(false)
            .fixed_size(vec2(200.0, f32::INFINITY))
            .show(ctx, |ui| {
                Grid::new(ui.id().with(state.monitor.id))
                    .num_columns(2)
                    .striped(true)
                    .min_col_width(100.0)
                    .spacing(vec2(20.0, 20.0))
                    .show(ui, |ui| {
                        //
                        // Name label
                        //
                        ui.label("Name").on_hover_text("Monitor name");

                        let name = egui::TextEdit::singleline(&mut state.name)
                            .hint_text(format!("Monitor {}", state.monitor.id))
                            .vertical_align(Align::Center);
                        ui.add_sized(vec2(100.0, ui.available_height()), name)
                            .on_hover_text("Monitor name");
                        ui.end_row();

                        //
                        // Separator
                        //
                        ui.separator();
                        ui.end_row();

                        //
                        // Resolution list
                        //
                        let mut refresh_rate_to_remove = None;
                        let mut mode_to_remove = None;
                        let mut refresh_to_add = None;
                        if let Some(modes) = &state.monitor.modes {
                            if !modes.is_empty() {
                                for (idx, (key, mode)) in modes.iter().enumerate() {
                                    //
                                    // Resolution label
                                    //
                                    ui.horizontal(|ui| {
                                        // Remove mode button
                                        let button = egui::Button::new("-")
                                            .fill(Color32::DARK_RED)
                                            .rounding(8.0);

                                        if ui.add(button).clicked() {
                                            mode_to_remove = Some(key.clone());
                                        }

                                        let text = if mode.pending {
                                            format!("*{}x{}", mode.width, mode.height)
                                        } else {
                                            format!("{}x{}", mode.width, mode.height)
                                        };

                                        ui.label(text);
                                    });

                                    //
                                    // Refresh rate list
                                    //
                                    CollapsingHeader::new("Refresh rates")
                                        .id_source(ui.id().with("res").with(idx))
                                        .show(ui, |ui| {
                                            if !mode.refresh_rates.is_empty() {
                                                Grid::new("res_grid").num_columns(2).show(
                                                    ui,
                                                    |ui| {
                                                        for (ridx, refresh_rate) in
                                                            mode.refresh_rates.iter().enumerate()
                                                        {
                                                            // text
                                                            let text = if refresh_rate.pending {
                                                                format!("*{}", refresh_rate.rate)
                                                            } else {
                                                                refresh_rate.rate.to_string()
                                                            };

                                                            ui.label(text);

                                                            // button
                                                            ui.allocate_ui(
                                                                [35.0, ui.available_height()]
                                                                    .into(),
                                                                |ui| {
                                                                    // button
                                                                    ui.with_layout(
                                                                        Layout::right_to_left(
                                                                            Align::Center,
                                                                        ),
                                                                        |ui| {
                                                                            let button =
                                                                            egui::Button::new("-")
                                                                                .fill(Color32::DARK_RED)
                                                                                .rounding(8.0);

                                                                            if ui.add(button).clicked()
                                                                            {
                                                                                refresh_rate_to_remove =
                                                                                Some((
                                                                                    key.clone(),
                                                                                    ridx,
                                                                                ));
                                                                            }
                                                                        },
                                                                    );
                                                                },
                                                            );

                                                            ui.end_row();
                                                        }
                                                    },
                                                );
                                            }

                                            ui.with_layout(
                                                Layout::right_to_left(Align::Center),
                                                |ui| {
                                                    if ui.button("+").clicked() && state.tmp_add > 0
                                                    {
                                                        refresh_to_add =
                                                            Some((key.clone(), state.tmp_add));
                                                    }

                                                    ui.with_layout(
                                                        Layout::left_to_right(Align::Center),
                                                        |ui| {
                                                            let mut add = if state.tmp_add > 0 {
                                                                state.tmp_add.to_string()
                                                            } else {
                                                                "".to_string()
                                                            };

                                                            let add_widget =
                                                                egui::TextEdit::singleline(
                                                                    &mut add,
                                                                )
                                                                .vertical_align(Align::Center)
                                                                .desired_width(40.0);

                                                            let res_add = ui.add(add_widget);

                                                            if res_add.changed() {
                                                                if let Ok(a) = add.parse::<u32>() {
                                                                    state.tmp_add = a;
                                                                } else if add.is_empty() {
                                                                    state.tmp_add = 0;
                                                                }
                                                            }
                                                        },
                                                    );
                                                },
                                            );
                                        });

                                    ui.end_row();
                                }
                            } else {
                                ui.colored_label(
                                    Color32::from_rgb(196, 166, 38),
                                    "No resolutions ⚠️",
                                )
                                .on_hover_text("Add a resolution to setup");
                                ui.end_row();
                            }
                        } else {
                            ui.colored_label(Color32::from_rgb(196, 166, 38), "No resolutions ⚠️")
                                .on_hover_text("Add a resolution to setup");
                            ui.end_row();
                        }

                        if let Some((key, ridx)) = refresh_rate_to_remove {
                            if let Some(modes) = &mut state.monitor.modes {
                                let mut remove_mode = false;
                                if let Some(mode) = modes.get_mut(&key) {
                                    mode.refresh_rates.remove(ridx);

                                    if mode.refresh_rates.is_empty() {
                                        remove_mode = true;
                                    }
                                }

                                // no more modes, so entire mode is dead, remove it
                                if remove_mode {
                                    modes.remove(&key);

                                    if modes.is_empty() {
                                        state.enabled = false;
                                        ipc_call(&mut self.con.borrow_mut(), DriverCommand::Remove(vec![state.monitor.id]));
                                    }
                                }
                            }
                        }

                        if let Some(mode) = mode_to_remove {
                            if let Some(modes) = &mut state.monitor.modes {
                                modes.remove(&mode);

                                // if this was the last mode, then it needs to be disabled and removed
                                if modes.is_empty() {
                                    state.enabled = false;
                                    ipc_call(&mut self.con.borrow_mut(), DriverCommand::Remove(vec![state.monitor.id]));
                                }
                            }
                        }

                        if let Some((key, val)) = refresh_to_add {
                            if let Some(modes) = &mut state.monitor.modes {
                                if let Some(rr) = modes.get_mut(&key) {
                                    // make sure rate does not already exist
                                    let exists = rr.refresh_rates.iter().any(|r| r.rate == val);

                                    if !exists {
                                        rr.refresh_rates.push(RefreshRate {
                                            rate: val,
                                            pending: true,
                                        });

                                        state.tmp_add = 0;
                                    }
                                }
                            }
                        }

                        //
                        // Save / clear section
                        //
                        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                            if ui.button("Clear").clicked() {
                                state.monitor.remove_pending();
                            }

                            //
                            // Add new monitor section
                            //

                            if ui.button("+").clicked() {
                                state.add_resolution_window = true;
                            }
                        });

                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if ui.button("Save").clicked() {
                                state.monitor.accept_pending();
                            }
                        });

                        ui.end_row();


                    });
            });

        if state.add_resolution_window {
            AddResolutionWindow::new().show(ctx, state);
        }
    }
}
