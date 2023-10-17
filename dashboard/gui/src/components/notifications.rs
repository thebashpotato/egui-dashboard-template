//! Notifications bar at the bottom of the app

use crate::app::State;
use dashboard_aesthetix::themes::Aesthetix;
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
    pub fn ui(
        &mut self,
        context: &egui::Context,
        state: &State,
    ) {
        let mut bottom_bar = egui::TopBottomPanel::bottom("bottom_panel").frame(
            egui::Frame::default()
                .fill(state.active_theme.bg_secondary_color_visuals())
                .inner_margin(egui::vec2(10.0, 5.0)),
        );

        let alignment = if !self.expanded {
            bottom_bar = bottom_bar.max_height(26.0);
            egui::Align::TOP
        } else {
            egui::Align::Center
        };

        bottom_bar.show(context, |ui| {
            ui.with_layout(egui::Layout::right_to_left(alignment), |ui| {
                if !self.expanded {
                    if ui.small_button("Expand").clicked() {
                        self.expanded = true;
                    }
                } else if ui.button("Reduce").clicked() {
                    self.expanded = false;
                }

                ui.with_layout(egui::Layout::left_to_right(alignment), |ui| {
                    ui.add(egui::Label::new(egui::RichText::new(&self.message)).wrap(true))
                });
            });
        });
    }
}
