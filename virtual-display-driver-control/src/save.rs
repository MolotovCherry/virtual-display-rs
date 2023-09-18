use std::fs;

use winreg::{
    enums::{HKEY_LOCAL_MACHINE, KEY_WRITE},
    RegKey,
};

use crate::app::App;

pub fn save_config(app: &App) {
    // write out app config
    let json = serde_json::to_string(app).unwrap();

    let parent = app.config.parent().unwrap();
    if !parent.exists() {
        _ = fs::create_dir_all(app.config.parent().unwrap());
    }

    fs::write(&app.config, json.as_bytes()).unwrap();

    // write out final config to registry for driver
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = "SOFTWARE\\VirtualDisplayDriver";

    let driver_settings = if let Ok(driver_settings) = hklm.open_subkey_with_flags(key, KEY_WRITE) {
        driver_settings
    } else {
        hklm.create_subkey(key).unwrap().0
    };

    let monitors = app
        .monitors
        .iter()
        .flat_map(|i| i.monitor.as_ref())
        .collect::<Vec<_>>();

    let data = serde_json::to_string(&monitors).unwrap();

    driver_settings.set_value("port", &app.port).unwrap();
    driver_settings
        .set_value("data", &if app.enabled { &data } else { "[]" })
        .unwrap();
}
