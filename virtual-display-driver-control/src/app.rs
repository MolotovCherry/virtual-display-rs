use std::{
    collections::HashMap,
    fs,
    net::TcpStream,
    ops::{Deref, DerefMut},
    path::PathBuf,
    sync::Arc,
};

use directories::ProjectDirs;
use driver_ipc::{DriverCommand, Monitor};
use egui::{vec2, Align, CentralPanel, Color32, Id, Layout, Margin, Rounding, Ui};
use serde::{Deserialize, Serialize};

use crate::{
    actions::Action,
    ipc::ipc_call,
    popup::{display_popup, MessageBoxIcon},
    save::save_config,
};

#[derive(Default, Debug)]
pub enum TcpWrapper {
    Connected(TcpStream),
    #[default]
    Disconnected,
}

impl Deref for TcpWrapper {
    type Target = TcpStream;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Connected(c) => c,
            _ => unreachable!(),
        }
    }
}

impl DerefMut for TcpWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Connected(c) => c,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    pub enabled: bool,
    pub port: u32,
    pub monitors: Vec<Arc<Monitor>>,
    #[serde(skip)]
    pub connection: TcpWrapper,
    #[serde(skip)]
    pub config: PathBuf,
    #[serde(skip)]
    pub actions: HashMap<u32, Action>,
}

impl Default for App {
    fn default() -> Self {
        let Some(dir) = ProjectDirs::from("", "", "Virtual Display Driver") else {
            panic!("Could not get project directory");
        };

        // load config.json from project directory
        let config = dir.config_dir().join("config.json");
        let app = 'ret: {
            if config.exists() {
                let Ok(app_config) = fs::read_to_string(&config) else {
                    break 'ret None;
                };

                let Ok(mut app) = serde_json::from_str::<App>(&app_config) else {
                    break 'ret None;
                };

                app.config = config.clone();

                let port = app.port;
                let stream = TcpStream::connect(format!("127.0.0.1:{port}"));

                let Ok(stream) = stream else {
                    display_popup(
                        "connection failure",
                        &format!("Failed to connect to driver at 127.0.0.1:{port}. If you just changed the port, the driver needs to be restarted"),
                        MessageBoxIcon::Error,
                    );
                    std::process::exit(1);
                };

                app.connection = TcpWrapper::Connected(stream);

                Some(app)
            } else {
                None
            }
        };

        let port = app.as_ref().map(|a| a.port).unwrap_or(23112u32);
        let stream = TcpStream::connect(format!("127.0.0.1:{port}"));
        let Ok(stream) = stream else {
            display_popup(
                "connection failure",
                &format!("failed to connect to driver at 127.0.0.1:{port}"),
                MessageBoxIcon::Error,
            );
            std::process::exit(1);
        };

        app.unwrap_or(Self {
            enabled: true,
            port: 23112u32,
            monitors: Default::default(),
            connection: TcpWrapper::Connected(stream),
            config,
            actions: HashMap::new(),
        })
    }
}

impl App {
    pub fn new() -> Self {
        App::default()
    }

    pub fn toggle_driver(&mut self) {
        if !self.enabled {
            ipc_call(self, DriverCommand::RemoveAll);
        } else {
            ipc_call(
                self,
                DriverCommand::Add(self.monitors.iter().map(|m| m.as_ref().clone()).collect()),
            );
        }

        save_config(self);
    }
}

impl eframe::App for App {
    fn on_close_event(&mut self) -> bool {
        save_config(self);
        true
    }

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let mut style: egui::Style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(10.0, 10.0);
        style.spacing.button_padding = vec2(10.0, 5.0);
        style.spacing.scroll_bar_inner_margin = 10.0;
        style.spacing.scroll_bar_outer_margin = 5.0;
        style.spacing.tooltip_width = 300.0;

        ctx.set_style(style.clone());

        let frame = egui::containers::Frame::none()
            .inner_margin(Margin {
                left: 5.0,
                right: 5.0,
                top: 5.0,
                bottom: 5.0,
            })
            .fill(Color32::from_gray(27));

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            egui::TopBottomPanel::top("top")
                .show_separator_line(false)
                .show_inside(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                            let checkbox = ui
                                .checkbox(&mut self.enabled, "")
                                .on_hover_text("Enable or disable all monitors");
                            if checkbox.clicked() {
                                self.toggle_driver();
                            };

                            ui.label("Enabled")
                                .labelled_by(checkbox.id)
                                .on_hover_text("Enable or disable all monitors");

                            port_edit(ui, &mut self.port);
                        });
                    });
                });

            CentralPanel::default().show_inside(ui, |ui| {
                let id = Id::new("scrollarea");

                let mut offset = 0.0;
                ui.ctx().data(|reader| {
                    offset = reader.get_temp::<f32>(id).unwrap_or(0.0);
                });

                let scroll_area = egui::ScrollArea::new([false, true])
                    .vertical_scroll_offset(offset)
                    .max_height(ui.available_height() - 30.0);

                let output = scroll_area.show(ui, |ui| {
                    egui::Grid::new("grid").show(ui, |ui| {
                        let mut peek = self.monitors.iter().enumerate().peekable();

                        while let Some((idx, monitor)) = peek.next() {
                            let button = egui::Button::new((monitor.id + 1).to_string())
                                .rounding(Rounding::same(8.0))
                                .min_size(vec2(200.0, 200.0));
                            ui.add(button);

                            // only 3 per row
                            if (idx + 1) % 3 == 0 {
                                ui.end_row();
                            }

                            if peek.peek().is_none() && self.monitors.len() < 10 {
                                let button = egui::Button::new("+")
                                    .rounding(Rounding::same(8.0))
                                    .min_size(vec2(200.0, 200.0));
                                ui.add(button);
                            }
                        }
                    });
                });

                ui.ctx().data_mut(|writer| {
                    writer.insert_temp(id, output.state.offset.y);
                });

                let id = Id::new("init");
                let mut initted = false;
                let mut size = ui.available_width();
                ui.ctx().data(|reader| {
                    initted = reader.get_temp::<bool>(id).unwrap_or(false);
                    size = reader.get_temp::<f32>(id).unwrap_or(ui.available_width());
                });

                if !initted {
                    ui.ctx().data_mut(|writer| {
                        writer.insert_temp(id, true);
                        writer.insert_temp(id, ui.available_width() + 88.0);
                    });
                }

                _frame.set_window_size(vec2(size, 500.0));
            });

            egui::TopBottomPanel::bottom("bottom")
                .show_separator_line(false)
                .show_inside(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                            let enabled = !self.actions.is_empty();
                            let button = egui::Button::new("Apply");
                            let res = ui
                                .add_enabled(enabled, button)
                                .on_hover_text("Apply all pending operations");
                            if res.clicked() {
                                save_config(self);
                            }

                            let button = egui::Button::new("Clear");
                            let res = ui
                                .add_enabled(enabled, button)
                                .on_hover_text("Clear all pending operations");
                            if res.clicked() {
                                self.actions.clear();
                            }
                        });
                    });
                });
        });
    }
}

fn port_edit(ui: &mut Ui, port: &mut u32) {
    ui.horizontal_wrapped(|ui| {
        let mut port_s = port.to_string();

        let port_widget = egui::TextEdit::singleline(&mut port_s);
        let res = ui.add_sized(vec2(75.0, 20.0), port_widget).on_hover_text(
            "Port driver listens on. Driver must be restarted for port change to take effect",
        );

        if res.changed() {
            if let Ok(port_p) = port_s.parse::<u32>() {
                *port = port_p;
            }
        };

        ui.label("Port").labelled_by(res.id).on_hover_text(
            "Port driver listens on. Driver must be restarted for port change to take effect",
        );
    });
}
