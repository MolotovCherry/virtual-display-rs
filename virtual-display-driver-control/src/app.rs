use std::{
    fs,
    net::TcpStream,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use directories::ProjectDirs;
use driver_ipc::Monitor;
use serde::{Deserialize, Serialize};
use winreg::{
    enums::{HKEY_LOCAL_MACHINE, KEY_WRITE},
    RegKey,
};

use crate::popup::{display_popup, MessageBoxIcon};

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
    pub monitors: Vec<Monitor>,
    #[serde(skip)]
    pub connection: TcpWrapper,
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
        let app = 'ret: {
            if config.exists() {
                let Ok(config) = fs::read_to_string(&config) else {
                    break 'ret None;
                };

                let Ok(app) = serde_json::from_str::<App>(&config) else {
                    break 'ret None;
                };

                Some(app)
            } else {
                None
            }
        };

        let port = app.as_ref().map(|a| a.port).unwrap_or(23112u32);
        let stream = TcpStream::connect(format!("127.0.0.1:{port}"));
        let Ok(stream) = stream else {
            display_popup(
                "can't connect",
                &format!("couldn't connect to driver at 127.0.0.1:{port}"),
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
        })
    }
}

impl App {
    pub fn new() -> Self {
        App::default()
    }
}

impl eframe::App for App {
    fn on_close_event(&mut self) -> bool {
        // write out app config
        let json = serde_json::to_string(self).unwrap();
        
        _ = fs::create_dir_all(self.config.parent().unwrap());
        fs::write(self.config.clone(), json.as_bytes()).unwrap();

        // write out final config to registry for driver
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let key = "SOFTWARE\\VirtualDisplayDriver";

        let Ok(driver_settings) = hklm.open_subkey_with_flags(key, KEY_WRITE) else {
            return true;
        };

        let data = serde_json::to_string(&self.monitors).unwrap();

        driver_settings.set_value("port", &self.port).unwrap();
        driver_settings.set_value("data", &data).unwrap();

        true
    }

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {}
}
