//! About page

use dashboard_aesthetix::version::DASHBOARD_VERSION;
use eframe::egui::{RichText, Ui};

/// Renders the about tab
pub fn about_tab_ui(ui: &mut Ui) {
    ui.label(RichText::new(format!("Dashboard v{}", *DASHBOARD_VERSION)).size(30.0));
    ui.label(r#"Dashboard template for modern cross platform applications"#);
    ui.add_space(10.0);
    ui.hyperlink_to(
        "Visit us on GitHub",
        "https://github.com/thebashpotato/egui-dashboard-template",
    );
    ui.hyperlink_to(
        "Latest release",
        "https://github.com/thebashpotato/egui-dashboard-template/releases",
    );
}
