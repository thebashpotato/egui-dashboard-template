//! Application state and business logic.
mod state;

use crate::components::notifications::NotificationBar;
use eframe::egui;
pub use state::{State, Tab};
use std::collections::BTreeMap;

/// Holds application state and implements the business logic.
#[derive(Debug)]
pub struct Dashboard {
    /// Holds the state of the application
    state: State,
    /// Tab labels and icons
    tab_labels: BTreeMap<Tab, &'static str>,
    /// Notifications bar
    notification_bar: NotificationBar,
}

impl Dashboard {
    /// Create a new application    
    #[must_use]
    pub fn new(creation_context: &eframe::CreationContext<'_>) -> Self {
        let state = State::new();

        // Initialize the custom theme/styles for egui
        creation_context
            .egui_ctx
            .set_style(state.active_theme.custom_style());
        println!("{}", state.active_theme.name());
        Self {
            state,
            tab_labels: [
                (Tab::Home, "🏠  Home"),
                (Tab::Settings, "⚙  Settings"),
                (Tab::Logs, "📝  Logs"),
                (Tab::Debug, "🐞  Debug"),
                (Tab::About, "ℹ  About"),
            ]
            .into_iter()
            .collect(),
            notification_bar: NotificationBar::new(),
        }
    }
}

impl eframe::App for Dashboard {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        self.notification_bar.ui(context, &self.state);

        egui::SidePanel::left("main_side_panel")
            .resizable(false)
            .frame(
                egui::Frame::none()
                    .fill(self.state.active_theme.bg_primary_color_visuals())
                    .inner_margin(self.state.active_theme.margin_style())
                    .stroke(egui::Stroke::new(
                        1.0,
                        self.state.active_theme.bg_primary_color_visuals(),
                    )),
            )
            .exact_width(160.0)
            .show(context, |ui_side_panel| {
                ui_side_panel.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Center),
                    |ui_layout| {
                        ui_layout.add_space(13.0);
                        ui_layout.heading(egui::RichText::new("Dashboard").size(25.0).strong());
                        egui::warn_if_debug_build(ui_layout);
                    },
                );

                ui_side_panel.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Min),
                    |ui_layout| {
                        for (tab, label) in &self.tab_labels {
                            ui_layout.selectable_value(&mut self.state.active_tab, *tab, *label);
                        }
                    },
                );
            });
    }
}
