//! About page

use dashboard_common::version::DASHBOARD_VERSION;
use eframe::egui;

/// Constant so we can dynamically center the contents slighly above the center of the frame
const UPPER_CENTER: f32 = 5.0;

/// Renders the about tab
pub fn about_tab_ui(ui_root: &mut egui::Ui) {
    let computed_upper_center = ui_root.ctx().screen_rect().height() / UPPER_CENTER;
    ui_root.add_space(computed_upper_center);
    ui_root.vertical_centered_justified(|ui_vertical_centered| {
        ui_vertical_centered.add(egui::Label::new(
            egui::RichText::new(format!("Dashboard v{}", *DASHBOARD_VERSION)).size(30.0),
        ));
        ui_vertical_centered.label("Dashboard template for modern cross platform applications");

        ui_vertical_centered.hyperlink_to(
            "Check out the code on GitHub",
            "https://github.com/thebashpotato/egui-dashboard-template",
        );
        ui_vertical_centered.hyperlink_to(
            "Latest release",
            "https://github.com/thebashpotato/egui-dashboard-template/releases",
        );
    });
}
