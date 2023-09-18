use driver_ipc::DriverCommand;
use egui::{vec2, Align, Color32, Context, Id, Sense, Ui};

use crate::{
    app::{App, MonitorState},
    ipc::ipc_call,
    save::save_config,
    toggle_switch::toggle,
};

use super::monitor_window::MonitorWindow;

pub struct MainWindow<'a> {
    app: &'a mut App,
}

impl<'a> MainWindow<'a> {
    pub fn new(app: &'a mut App) -> Self {
        Self { app }
    }

    pub fn show(mut self, ctx: &Context) {
        egui::Window::new("Monitor Settings")
            .constrain(true)
            .auto_sized()
            .collapsible(false)
            .show(ctx, |ui| self.ui(ctx, ui));
    }

    fn ui(&mut self, ctx: &Context, ui: &mut Ui) {
        egui::Grid::new("grid")
            .striped(true)
            .spacing(vec2(80.0, 20.0))
            .show(ui, |ui| {
                //
                // Enable / Disable all monitors
                //
                ui.label("Enabled")
                    .on_hover_text("Enable/disable all monitors");

                let switch = ui
                    .add(toggle(&mut self.app.enabled))
                    .on_hover_text("Enable/disable all monitors");

                if anim_bool_finished(ui, switch.id.with("anim"), switch.clicked()) {
                    self.app.toggle_driver();
                }

                ui.end_row();

                //
                // Port selector
                //

                let mut port_s = self.app.port.to_string();

                ui.label("Port")
                    .on_hover_text("Port driver listens on. Port changes require a driver restart");

                let port_widget =
                    egui::TextEdit::singleline(&mut port_s).vertical_align(Align::Center);

                let res = ui
                    .add_sized(vec2(75.0, 20.0), port_widget)
                    .on_hover_text("Port driver listens on. Port changes require a driver restart");

                if res.changed() {
                    if let Ok(port_p) = port_s.parse::<u32>() {
                        self.app.port = port_p;
                    }
                };

                ui.end_row();

                //
                // Monitor list
                //

                ui.separator();

                // Paint the first element of the monitor list. This has to be done in advance cause of immediate mode
                if !self.app.monitors.is_empty() {
                    // grab the height of this row to compare after
                    let mut rect = ui.min_rect();

                    // compare to this one
                    let height = ui.min_rect().height();
                    rect.set_top(rect.top() + height + 10.0);
                    rect.set_bottom(rect.bottom() + (height / 2.0) + 6.0);
                    rect.set_left(rect.left() - 2.0);
                    rect.set_right(rect.right() + 2.0);

                    let response = ui.interact(rect, Id::new("rect1"), Sense::click());
                    if response.clicked() || self.app.monitors[0].monitor_window {
                        self.app.monitors[0].monitor_window = true;
                        MonitorWindow::new().show(ctx, &mut self.app.monitors[0]);
                    }

                    // Paint a color on hover / not hover
                    let color = if response.hovered() {
                        Color32::from_gray(50)
                    } else {
                        Color32::from_gray(27)
                    };

                    ui.painter().rect_filled(rect, 2.0, color);
                }

                ui.end_row();

                let mut idx_to_remove = None;

                let mut cumulative_rect = ui.min_rect();

                for idx in 0..self.app.monitors.len() {
                    //
                    // Row painting / highlighting
                    // This has to be done in advance, so these are painting the NEXT row (not the current iteration)
                    //
                    // This never paints the next one if there isn't one, because our iteration 0..self.app.monitors.len() stops that
                    //

                    let mut rect = ui.min_rect();

                    // get the difference by setting the top of this one to the bottom of the last
                    rect.set_top(cumulative_rect.bottom() + 10.0);
                    rect.set_bottom(rect.bottom() + 10.0);
                    rect.set_left(rect.left() - 2.0);
                    rect.set_right(rect.right() + 2.0);

                    // now shift the rect down by one by calculating the size
                    let height = rect.size().y;
                    rect.set_top(rect.top() + height);
                    rect.set_bottom(rect.bottom() + height);

                    let response = ui.interact(rect, Id::new("rect").with(idx + 1), Sense::click());
                    let state = &mut self.app.monitors[idx];
                    if response.clicked() || state.monitor_window && idx > 0 {
                        state.monitor_window = true;
                        MonitorWindow::new().show(ctx, state);
                    }

                    // Paint a color on hover / not hover
                    let color = if response.hovered() {
                        Color32::from_gray(50)
                    } else if (idx + 1) % 2 == 0 {
                        Color32::from_gray(27)
                    } else {
                        Color32::from_gray(32)
                    };

                    ui.painter().rect_filled(rect, 2.0, color);

                    // update the full one for the next iter
                    cumulative_rect = ui.min_rect();

                    //
                    // Cell contents
                    //

                    let state = &mut self.app.monitors[idx];

                    if state.monitor.is_none() {
                        ui.colored_label(
                            Color32::from_rgb(196, 166, 38),
                            if !state.name.is_empty() {
                                state.name.clone() + " ⚠️"
                            } else {
                                format!("Monitor {} ⚠️", state.id)
                            },
                        )
                        .on_hover_text("This monitor requires setup");
                    } else {
                        ui.label(if !state.name.is_empty() {
                            state.name.clone()
                        } else {
                            format!("Monitor {}", state.id)
                        });
                    }

                    //
                    // Delete monitor button
                    //
                    ui.horizontal_centered(|ui| {
                        let button = egui::Button::new("-").fill(Color32::DARK_RED).rounding(8.0);
                        let button_add = ui.add(button);

                        if button_add.clicked() {
                            state.delete_window = true;
                        }

                        if state.delete_window {
                            egui::Window::new(if !state.name.is_empty() {
                                format!("Delete {}?", state.name)
                            } else {
                                format!("Delete Monitor {}?", state.id)
                            })
                            .open(&mut state.delete_window)
                            .auto_sized()
                            .collapsible(false)
                            .show(ctx, |ui| {
                                egui::Grid::new("grid").show(ui, |ui| {
                                    if ui.button("Ok").clicked() {
                                        idx_to_remove = Some(idx);
                                    }
                                });
                            });
                        }

                        //
                        // Enable/disable monitor switch
                        //
                        let switch = ui
                            .add(toggle(&mut state.enabled))
                            .on_hover_text("Enable/disable monitor");

                        if anim_bool_finished(ui, switch.id.with("anim"), switch.clicked()) {
                            let state = &state;

                            // if monitor is set up, then add or remove it
                            if let Some(monitor) = &state.monitor {
                                if state.enabled {
                                    ipc_call(
                                        &mut self.app.connection.borrow_mut(),
                                        DriverCommand::Add(vec![monitor.as_ref().clone()]),
                                    )
                                } else {
                                    ipc_call(
                                        &mut self.app.connection.borrow_mut(),
                                        DriverCommand::Remove(vec![monitor.id]),
                                    )
                                }
                            }
                        }
                    });

                    ui.end_row();
                }

                // remove monitor if requested
                if let Some(idx) = idx_to_remove {
                    self.app.monitors.remove(idx);
                }

                let should_add_plus = self.app.monitors.len() < 10;
                if should_add_plus {
                    if ui.button("+").clicked() {
                        let mut id = self.app.monitors.len() + 1;
                        for (idx, mon) in self.app.monitors.iter().enumerate() {
                            let idx = idx + 1;
                            if mon.id != idx as u32 {
                                id = idx;
                                break;
                            }
                        }

                        self.app.monitors.insert(
                            id.saturating_sub(1),
                            MonitorState {
                                enabled: true,
                                id: id as u32,
                                ..Default::default()
                            },
                        );
                    }
                } else {
                    ui.separator();
                }

                if ui.button("Save").clicked() {
                    save_config(self.app);
                }
            });
    }
}

/// Returns if animation for condition is finished
fn anim_bool_finished(ui: &mut Ui, id: Id, cond: bool) -> bool {
    let result = ui.ctx().animate_bool(id, cond);
    if cond {
        ui.data_mut(|data| data.insert_temp(id, true));
        return false;
    }

    // wait until animation done before toggling
    if (result == 0.0 || result == 1.0) && ui.ctx().data(|data| data.get_temp(id).unwrap_or(false))
    {
        ui.data_mut(|data| data.insert_temp(id, false));
        return true;
    };

    false
}
