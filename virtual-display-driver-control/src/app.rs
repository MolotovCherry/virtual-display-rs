use std::{fs, net::TcpStream, path::PathBuf};

use directories::ProjectDirs;
use driver_ipc::Monitor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct App {
    enabled: bool,
    port: u32,
    monitors: Vec<Monitor>,
    #[serde(skip)]
    connection: Option<TcpStream>,
    #[serde(skip)]
    config: PathBuf,
}

impl App {
    pub fn new() -> Self {
        let Some(dir) = ProjectDirs::from("", "", "virtual-display-driver") else {
            panic!("Could not get project directory");
        };

        // load config.json from project directory
        let config = dir.config_dir().join("config.json");
        if config.exists() {
            let Ok(config) = fs::read_to_string(config) else {
                return App::default();
            };

            let Ok(app) = serde_json::from_str(&config) else {
                return App::default();
            };

            app
        } else {
            App::default()
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        todo!()
    }
}
