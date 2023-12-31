//! Application state and business logic.
mod state;

use crate::components;
use crate::components::notifications::NotificationBar;
use egui_aesthetix::{
    themes::{CarlDark, StandardDark, StandardLight},
    Aesthetix,
};

use eframe::egui;
pub use state::{ApplicationState, Tab};
use std::collections::BTreeMap;
use std::rc::Rc;

/// Holds application state and implements the business logic.
#[derive(Debug)]
pub struct Dashboard {
    /// Holds the state of the application
    state: ApplicationState,
    /// Tab labels and icons
    tab_labels: BTreeMap<Tab, &'static str>,
    /// Holds the supported themes that the user can switch between
    themes: Vec<Rc<dyn Aesthetix>>,
    /// Notifications bar
    notification_bar: NotificationBar,
}

impl Dashboard {
    /// Create a new application    
    ///
    /// # Panics
    ///
    /// if the first theme in the list of themes could not be loaded
    #[must_use]
    pub fn new(creation_context: &eframe::CreationContext<'_>) -> Self {
        let themes: Vec<Rc<dyn Aesthetix>> = vec![
            Rc::new(StandardDark),
            Rc::new(StandardLight),
            Rc::new(CarlDark),
        ];

        let active_theme: Rc<dyn Aesthetix> = match themes.first() {
            Some(theme) => theme.clone(),
            None => panic!(
                "The first theme in the list of available themes could not be loaded => 'Dashboard::new'"
            ),
        };

        let state = ApplicationState::new(active_theme);

        // Initialize the custom theme/styles for egui
        creation_context
            .egui_ctx
            .set_style(state.active_theme.custom_style());

        println!("Loading with '{}' theme", state.active_theme.name());

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
            themes,
            notification_bar: NotificationBar::new(),
        }
    }
}

impl eframe::App for Dashboard {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        // This builds the notifications bar at the bottom of the app
        // it must be drawn first so it stretches across the entire width of the app.
        self.notification_bar.ui(context, &self.state);

        // This builds the main side navigation panel
        egui::SidePanel::left("main_side_panel")
            .resizable(false)
            .frame(
                egui::Frame::none()
                    .fill(self.state.active_theme.bg_secondary_color_visuals())
                    .inner_margin(self.state.active_theme.margin_style())
                    .stroke(egui::Stroke::new(
                        1.0,
                        self.state.active_theme.bg_secondary_color_visuals(),
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

        // This builds the main central panel that holds the content of the active tab
        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .inner_margin(self.state.active_theme.margin_style())
                    .fill(self.state.active_theme.bg_primary_color_visuals()),
            )
            .show(context, |ui_central_panel| {
                ui_central_panel.add_space(13.0);
                ui_central_panel.heading(
                    egui::RichText::new(
                        *self
                            .tab_labels
                            .get(&self.state.active_tab)
                            .expect("Could not fetch the current active tab name"),
                    )
                    .size(25.0),
                );

                match self.state.active_tab {
                    Tab::Home => {
                        // Load the Home page
                        ui_central_panel.add_space(13.0);
                        ui_central_panel.heading(egui::RichText::new("Home").size(25.0));
                    }
                    Tab::Settings => {
                        // Load the Settings page
                        components::settings::settings_tab_ui(
                            ui_central_panel,
                            &mut self.state,
                            &self.themes,
                        );
                    }
                    Tab::Logs => {
                        // Load the Logs page
                        ui_central_panel.add_space(13.0);
                        ui_central_panel.heading(egui::RichText::new("Logs").size(25.0));
                    }
                    Tab::Debug => {
                        // Load the Debug page
                        ui_central_panel.add_space(13.0);
                        ui_central_panel.heading(egui::RichText::new("Debug").size(25.0));
                    }
                    Tab::About => {
                        // Load the About page
                        components::about::about_tab_ui(ui_central_panel);
                    }
                }
            });
    }
}
