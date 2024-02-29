#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use dashboard_common::version::DASHBOARD_VERSION;
use dashboard_gui::Dashboard;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let viewport = egui::ViewportBuilder::default()
        .with_title("Dashboard Template")
        .with_min_inner_size([795.0, 435.0]);

    eframe::run_native(
        &format!("Dashboard v{}", *DASHBOARD_VERSION),
        eframe::NativeOptions {
            viewport,
            centered: true,
            ..Default::default()
        },
        Box::new(move |creation_context| Box::new(Dashboard::new(creation_context))),
    )
}
