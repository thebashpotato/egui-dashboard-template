//! Custom button group for basic components

use super::DisplayString;
use eframe::egui::Ui;

/// Lets us know if a custom button group was clicked.
///
/// # Arguments
///     ui - The ui to draw the button group on.
///     options - The options to display in the button group.
///     selection - The current selection of the button group.
pub fn button_group_clicked(
    ui: &mut Ui,
    options: &[DisplayString],
    selection: &mut String,
) -> bool {
    let mut clicked = false;
    for id in options {
        let res = ui.selectable_value(selection, (**id).clone(), &id.display);
        if res.clicked() {
            clicked = true;
        }

        if cfg!(debug_assertions) {
            res.on_hover_text((**id).clone());
        }
    }
    clicked
}
