use driver_ipc::DriverCommand;
use egui::{vec2, Align, Color32, Context, Id, Layout, Sense, Ui};

use crate::{
    app::App,
    ipc::ipc_call,
    monitor::{Monitor, MonitorState},
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
                    .with_layout(Layout::right_to_left(Align::Center), |ui| {
                        // alligns it up with the others
                        ui.allocate_exact_size(
                            vec2(0.0, ui.available_height()),
                            Sense::focusable_noninteractive(),
                        );

                        ui.add(toggle(&mut self.app.enabled))
                            .on_hover_text("Enable/disable all monitors")
                    })
                    .inner;

                if anim_bool_finished(ui, switch.id.with("anim"), switch.clicked()) {
                    self.app.toggle_driver();
                }

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
                        MonitorWindow::new(&self.app.pipe).show(ctx, &mut self.app.monitors[0]);
                    }

                    // Paint a color on hover / not hover
                    if response.hovered() {
                        ui.painter().rect_filled(rect, 2.0, Color32::from_gray(50));
                    }
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
                        MonitorWindow::new(&self.app.pipe).show(ctx, state);
                    }

                    // Paint a color on hover
                    if response.hovered() {
                        ui.painter().rect_filled(rect, 2.0, Color32::from_gray(50));
                    }

                    // update the full one for the next iter
                    cumulative_rect = ui.min_rect();

                    //
                    // Cell contents
                    //

                    let state = &mut self.app.monitors[idx];

                    let needs_setup = if let Some(modes) = &state.monitor.modes {
                        // len is 0
                        modes.is_empty()
                        // or, if all resolutions are pending or all refresh rates of that resolution are pending
                            || modes.iter().all(|(_, mode)| {
                                mode.pending || mode.refresh_rates.iter().all(|r| r.pending)
                            })
                    } else {
                        // it's empty, definitely needs setup
                        true
                    };

                    if needs_setup {
                        ui.colored_label(
                            Color32::from_rgb(196, 166, 38),
                            if !state.name.is_empty() {
                                state.name.clone() + " ⚠️"
                            } else {
                                format!("Monitor {} ⚠️", state.monitor.id)
                            },
                        )
                        .on_hover_text("This monitor requires setup");
                    } else {
                        ui.label(if !state.name.is_empty() {
                            state.name.clone()
                        } else {
                            format!("Monitor {}", state.monitor.id)
                        });
                    }

                    //
                    // Delete monitor button
                    //
                    ui.horizontal_centered(|ui| {
                        let button = egui::Button::new("-").fill(Color32::DARK_RED).rounding(8.0);
                        let button_add = ui.add(button);

                        if button_add.clicked() {
                            idx_to_remove = Some(idx);

                            if state.enabled
                                && state.monitor.modes.as_ref().is_some_and(|l| !l.is_empty())
                            {
                                ipc_call(
                                    &mut self.app.pipe.borrow_mut(),
                                    DriverCommand::Remove(vec![state.monitor.id]),
                                )
                            }
                        }

                        //
                        // Enable/disable monitor switch
                        //
                        let switch = ui
                            .add_enabled(!needs_setup, toggle(&mut state.enabled))
                            .on_hover_text("Enable/disable monitor");

                        if anim_bool_finished(ui, switch.id.with("anim"), switch.clicked()) {
                            // if monitor is set up, then add or remove it
                            if state.monitor.modes.as_ref().is_some_and(|l| !l.is_empty()) {
                                // allow monitor to enable only if monitor AND global switch is on, but it WILL turn off a monitor if it was on
                                if state.enabled && self.app.enabled {
                                    ipc_call(
                                        &mut self.app.pipe.borrow_mut(),
                                        DriverCommand::Add(vec![state.monitor.clone().into()]),
                                    )
                                } else if !state.enabled {
                                    ipc_call(
                                        &mut self.app.pipe.borrow_mut(),
                                        DriverCommand::Remove(vec![state.monitor.id]),
                                    )
                                }
                            }
                        }
                    });

                    ui.end_row();
                }

                // if no monitors are here, display a nice message
                if self.app.monitors.is_empty() {
                    ui.colored_label(Color32::from_rgb(196, 166, 38), "No monitors ⚠️")
                        .on_hover_text("Click the + button to add a monitor");
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
                        for (idx, state) in self.app.monitors.iter().enumerate() {
                            let idx = idx + 1;
                            if state.monitor.id != idx as u32 {
                                id = idx;
                                break;
                            }
                        }

                        self.app.monitors.insert(
                            id.saturating_sub(1),
                            MonitorState {
                                enabled: false,
                                monitor: Monitor {
                                    id: id as u32,
                                    modes: None,
                                },
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
fn anim_bool_finished(ui: &Ui, id: Id, cond: bool) -> bool {
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
