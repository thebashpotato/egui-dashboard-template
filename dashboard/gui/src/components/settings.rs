//! Settings Tab

use crate::app::ApplicationState;
use eframe::egui;
use egui_aesthetix::Aesthetix;
use std::rc::Rc;

/// Renders the about tab
pub fn settings_tab_ui(
    ui_root: &mut egui::Ui,
    state: &mut ApplicationState,
    themes: &Vec<Rc<dyn Aesthetix>>,
) {
    ui_root.horizontal_centered(|ui_horizontal_center| {
        // add combo box to allow the user to select from the available themes in the themes vector
        ui_horizontal_center.label("Theme:");
        egui::ComboBox::from_id_source("settings_theme_combo_box")
            .width(200.0)
            .selected_text(state.active_theme.name())
            .show_ui(ui_horizontal_center, |ui_combobox| {
                for theme in themes.iter() {
                    let res: egui::Response = ui_combobox.selectable_value(
                        &mut state.active_theme,
                        theme.clone(),
                        theme.name(),
                    );
                    if res.changed() {
                        println!("Theme changed to '{}'", theme.name());
                        ui_combobox
                            .ctx()
                            .set_style(state.active_theme.custom_style());
                    }
                }
            });
    });
}
