#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
#[cfg(debug_assertions)]
mod backtrace;
mod ipc;
mod load_icon;
mod monitor;
mod panic;
mod popup;
mod save;
mod toggle_switch;
mod ui;

use std::error::Error;

use eframe::{epaint::Vec2, NativeOptions};
use panic::set_hook;

use self::load_icon::load_app_icon;

fn main() -> Result<(), Box<dyn Error>> {
    set_hook();

    let options = NativeOptions {
        //min_window_size: Some(Vec2::new(500.0, 400.0)),
        icon_data: Some(load_app_icon()),
        initial_window_size: Some(Vec2::new(1000.0, 800.0)),
        transparent: true,
        resizable: true,
        centered: true,
        decorated: true,
        ..Default::default()
    };

    eframe::run_native(
        "Virtual Display Driver Control",
        options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    )?;

    Ok(())
}
