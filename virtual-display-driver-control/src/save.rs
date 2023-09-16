use std::fs;

use winreg::{
    enums::{HKEY_LOCAL_MACHINE, KEY_WRITE},
    RegKey,
};

use crate::app::App;

pub fn save(app: &App) {
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

    let Ok(driver_settings) = hklm.open_subkey_with_flags(key, KEY_WRITE) else {
        return;
    };

    let data = serde_json::to_string(&app.monitors).unwrap();

    driver_settings.set_value("port", &app.port).unwrap();
    driver_settings.set_value("data", &data).unwrap();
}
