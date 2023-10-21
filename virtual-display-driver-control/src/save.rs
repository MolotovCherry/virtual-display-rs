use std::fs;

use winreg::{
    enums::{HKEY_LOCAL_MACHINE, KEY_WRITE},
    RegKey,
};

use crate::{
    app::{App, PipeWrapper},
    monitor::RemovePending,
};

pub fn save_config(app: &App) {
    // effective clone, but with all pending removed. we don't want to persist pending
    let app = App {
        enabled: app.enabled,
        monitors: app.monitors.clone().remove_pending(),
        pipe: PipeWrapper::Disconnected.into(),
        config: app.config.clone(),
    };

    // write out app config
    let json = serde_json::to_string(&app).unwrap();

    let parent = app.config.parent().unwrap();
    if !parent.exists() {
        _ = fs::create_dir_all(app.config.parent().unwrap());
    }

    fs::write(&app.config, json.as_bytes()).unwrap();

    // write out final config to registry for driver
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = r"SOFTWARE\VirtualDisplayDriver";

    let driver_settings = if let Ok(driver_settings) = hklm.open_subkey_with_flags(key, KEY_WRITE) {
        driver_settings
    } else {
        hklm.create_subkey(key).unwrap().0
    };

    let monitors = app
        .monitors
        .iter()
        .map(|state| state.monitor.clone().into())
        .collect::<Vec<driver_ipc::Monitor>>();

    let data = serde_json::to_string(&monitors).unwrap();

    driver_settings
        .set_value("data", &if app.enabled { &data } else { "[]" })
        .unwrap();
}
