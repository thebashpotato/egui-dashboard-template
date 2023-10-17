//! Notifications bar at the bottom of the app

use crate::app::State;
use eframe::egui;

// Text to display when there are no notifications
const NO_NOTIFICATIONS: &str = "No new notifications";

/// Renders the notifications bar at the bottom of the app
#[derive(Debug)]
pub struct NotificationBar {
    /// Message to display in the notifications bar
    message: String,
    /// Expand the bottom bar
    expanded: bool,
}

impl NotificationBar {
    /// Renders the notifications bar at the bottom of the app
    pub fn new() -> Self {
        Self {
            message: NO_NOTIFICATIONS.to_owned(),
            expanded: false,
        }
    }

    /// Renders the bottom bar
    pub fn ui(&mut self, context: &egui::Context, state: &State) {
        let mut bottom_panel_widget = egui::TopBottomPanel::bottom("bottom_panel").frame(
            egui::Frame::default()
                .fill(state.active_theme.bg_secondary_color_visuals())
                .inner_margin(egui::vec2(10.0, 5.0)),
        );

        let alignment = if !self.expanded {
            bottom_panel_widget = bottom_panel_widget.max_height(26.0);
            egui::Align::TOP
        } else {
            egui::Align::Center
        };

        bottom_panel_widget.show(context, |ui_panel| {
            ui_panel.with_layout(egui::Layout::right_to_left(alignment), |ui_panel_layout| {
                if !self.expanded {
                    if ui_panel_layout.small_button("Expand").clicked() {
                        self.expanded = true;
                    }
                } else if ui_panel_layout.button("Reduce").clicked() {
                    self.expanded = false;
                }

                ui_panel_layout.with_layout(
                    egui::Layout::left_to_right(alignment),
                    |ui_panel_layout_label| {
                        ui_panel_layout_label
                            .add(egui::Label::new(egui::RichText::new(&self.message)).wrap(true))
                    },
                );
            });
        });
    }
}
