#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use dashboard_common::version::DASHBOARD_VERSION;
use dashboard_gui::Dashboard;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    eframe::run_native(
        &format!("Dashboard v{}", *DASHBOARD_VERSION),
        eframe::NativeOptions {
            min_window_size: Some(egui::vec2(600.0, 400.0)),
            initial_window_size: Some(egui::vec2(870.0, 600.0)),
            centered: true,
            ..Default::default()
        },
        Box::new(move |creation_context| Box::new(Dashboard::new(creation_context))),
    )
}
