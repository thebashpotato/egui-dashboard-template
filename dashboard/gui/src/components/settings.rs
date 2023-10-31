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
    egui::ScrollArea::new([false, true])
        .id_source("settings_tab_scroll_area")
        .show(ui_root, |ui_scroll_area| {
            ui_scroll_area.add_space(20.0);

            ui_scroll_area.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui_layout| {
                    egui::Grid::new("settings_grid")
                        .striped(true)
                        .num_columns(2)
                        .show(ui_layout, |ui_grid| {
                            ui_grid.add(egui::Label::new(
                                egui::RichText::new("Theme").size(18.0).monospace(),
                            ));

                            egui::ComboBox::from_id_source("settings_theme_combo_box")
                                .width(200.0)
                                .selected_text(state.active_theme.name())
                                .show_ui(ui_grid, |ui_combobox| {
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

                            ui_grid.end_row();
                        });
                },
            );
        });
}
