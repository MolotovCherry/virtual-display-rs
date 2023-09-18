use std::{
    cell::RefCell,
    collections::HashMap,
    fs,
    net::TcpStream,
    ops::{Deref, DerefMut},
    path::PathBuf,
    sync::Arc,
};

use directories::ProjectDirs;
use driver_ipc::{DriverCommand, Monitor};

use eframe::CreationContext;
use egui::vec2;

use serde::{Deserialize, Serialize};

use crate::{
    actions::Action,
    ipc::ipc_call,
    popup::{display_popup, MessageBoxIcon},
    save::save_config,
    ui::main_window::MainWindow,
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
    pub monitors: Vec<MonitorState>,
    #[serde(skip)]
    pub connection: RefCell<TcpWrapper>,
    #[serde(skip)]
    pub config: PathBuf,
    #[serde(skip)]
    pub actions: HashMap<u32, Action>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MonitorState {
    pub name: String,
    // whether monitor is enabled or not
    pub enabled: bool,
    pub id: u32,
    #[serde(skip)]
    pub monitor_window: bool,
    // when a monitor is first added, it is not initialized
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<Arc<Monitor>>,
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

                app.connection = RefCell::new(TcpWrapper::Connected(stream));

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
            connection: RefCell::new(TcpWrapper::Connected(stream)),
            config,
            actions: HashMap::new(),
        })
    }
}

impl App {
    pub fn new(cc: &CreationContext) -> Self {
        setup(&cc.egui_ctx);
        App::default()
    }

    pub fn toggle_driver(&self) {
        if !self.enabled {
            ipc_call(&mut self.connection.borrow_mut(), DriverCommand::RemoveAll);
        } else {
            ipc_call(
                &mut self.connection.borrow_mut(),
                DriverCommand::Add(
                    self.monitors
                        .iter()
                        .flat_map(|m| m.monitor.as_ref().map(|m| m.as_ref().clone()))
                        .collect(),
                ),
            );
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
