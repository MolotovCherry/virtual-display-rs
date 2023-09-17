#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod actions;
mod app;
#[cfg(debug_assertions)]
mod backtrace;
mod ipc;
mod panic;
mod popup;
mod save;
mod ui;

use std::error::Error;

use eframe::{epaint::Vec2, NativeOptions};
use panic::set_hook;

fn main() -> Result<(), Box<dyn Error>> {
    set_hook();

    let app = app::App::new();

    let options = NativeOptions {
        //min_window_size: Some(Vec2::new(500.0, 400.0)),
        icon_data: None,
        initial_window_size: Some(Vec2::new(600.0, 400.0)),
        transparent: false,
        resizable: false,
        centered: true,
        decorated: true,
        ..Default::default()
    };

    eframe::run_native(
        "Virtual Display Driver Control",
        options,
        Box::new(|_cc| Box::new(app)),
    )?;

    Ok(())
}
