//! About page

use dashboard_common::version::DASHBOARD_VERSION;
use eframe::egui;

/// Constant so we can dynamically center the About test slighly above the center of the screen
const UPPER_CENTER: f32 = 5.0;

/// Renders the about tab
pub fn about_tab_ui(ui: &mut egui::Ui) {
    let computed_upper_center = ui.ctx().screen_rect().height() / UPPER_CENTER;
    ui.add_space(computed_upper_center);
    ui.vertical_centered_justified(|ui_horizontal_center| {
        ui_horizontal_center.add(egui::Label::new(
            egui::RichText::new(format!("Dashboard v{}", *DASHBOARD_VERSION)).size(30.0),
        ));
        ui_horizontal_center.label(r#"Dashboard template for modern cross platform applications"#);

        ui_horizontal_center.hyperlink_to(
            "Check out the code on GitHub",
            "https://github.com/thebashpotato/egui-dashboard-template",
        );
        ui_horizontal_center.hyperlink_to(
            "Latest release",
            "https://github.com/thebashpotato/egui-dashboard-template/releases",
        );
    });
}
