use std::{
    cell::RefCell,
    fs,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use directories::ProjectDirs;
use driver_ipc::DriverCommand;

use eframe::CreationContext;
use egui::vec2;

use serde::{Deserialize, Serialize};
use win_pipes::{NamedPipeClient, NamedPipeClientOptions};

use crate::{
    ipc::ipc_call,
    monitor::{IntoIpc, MonitorState},
    popup::{display_popup, MessageBoxIcon},
    save::save_config,
    ui::main_window::MainWindow,
};

#[derive(Default, Debug)]
pub enum PipeWrapper {
    Client(NamedPipeClient),
    #[default]
    Disconnected,
}

impl Deref for PipeWrapper {
    type Target = NamedPipeClient;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Client(c) => c,
            _ => unreachable!(),
        }
    }
}

impl DerefMut for PipeWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Client(c) => c,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    pub enabled: bool,
    pub monitors: Vec<MonitorState>,
    #[serde(skip)]
    pub pipe: RefCell<PipeWrapper>,
    #[serde(skip)]
    pub config: PathBuf,
}

impl Default for App {
    fn default() -> Self {
        let Some(dir) = ProjectDirs::from("", "", "Virtual Display Driver") else {
            panic!("Could not get project directory");
        };

        // load config.json from project directory
        let config = dir.config_dir().join("config.json");

        let client = NamedPipeClientOptions::new(r"\\.\pipe\virtualdisplaydriver")
            .access_outbound()
            .wait()
            .mode_message()
            .create()
            .unwrap();

        if config.exists() {
            let Ok(app_config) = fs::read_to_string(&config) else {
                display_popup(
                    "connection failure",
                    r"Failed to read config to string. Is the config correct?",
                    MessageBoxIcon::Error,
                );
                std::process::exit(1);
            };

            let Ok(mut app) = serde_json::from_str::<App>(&app_config) else {
                display_popup(
                    "connection failure",
                    r"Failed to deserialize config. Is the config correct?",
                    MessageBoxIcon::Error,
                );
                std::process::exit(1);
            };

            app.config = config;

            app.pipe = RefCell::new(PipeWrapper::Client(client));

            app
        } else {
            Self {
                enabled: true,

                monitors: Default::default(),
                pipe: RefCell::new(PipeWrapper::Client(client)),
                config,
            }
        }
    }
}

impl App {
    pub fn new(cc: &CreationContext) -> Self {
        setup(&cc.egui_ctx);
        App::default()
    }

    pub fn toggle_driver(&self) {
        if !self.enabled {
            // only make ipc call if any monitors are actually enabled
            let enabled = self.monitors.iter().any(|s| s.enabled);

            if enabled {
                ipc_call(&mut self.pipe.borrow_mut(), DriverCommand::RemoveAll);
            }
        } else {
            // this removes pending monitors and monitors that are not enabled
            let monitors = self.monitors.clone().into_monitors_enabled();

            if !monitors.is_empty() {
                ipc_call(&mut self.pipe.borrow_mut(), DriverCommand::Add(monitors));
            }
        }

        save_config(self);
    }
}

fn setup(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // this cannot be redistributed due to copyright
    let font =
        std::fs::read("C:/Windows/Fonts/seguiemj.ttf").expect("Windows emoji font not found");

    fonts
        .font_data
        .insert("emoji".to_owned(), egui::FontData::from_owned(font));

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .push("emoji".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("emoji".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);

    let mut style: egui::Style = (*ctx.style()).clone();
    style.spacing.button_padding = vec2(10.0, 5.0);
    style.spacing.scroll_bar_inner_margin = 10.0;
    style.spacing.scroll_bar_outer_margin = 5.0;
    style.spacing.tooltip_width = 300.0;

    // style.debug.show_blocking_widget = true;
    // style.debug.show_interactive_widgets = true;

    ctx.set_style(style.clone());
}

impl eframe::App for App {
    fn on_close_event(&mut self) -> bool {
        save_config(self);
        true
    }

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        MainWindow::new(self).show(ctx);
    }
}
