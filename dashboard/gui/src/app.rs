//! Application state and business logic.

use dashboard_template_common::theme;
use eframe::egui;

/// Application state
pub struct State {
    /// dummy state
    name: String,
    /// dummy state
    age: u32,
    /// theme
    active_theme: theme::Catppuccin,
    /// layout style
    layout: theme::Layout,
    /// is the theme initialized
    theme_initialized: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            name: "John Doe".to_owned(),
            age: 42,
            active_theme: theme::MACCHIATO,
            layout: theme::Layout::default(),
            theme_initialized: false,
        }
    }
}

/// Holds application state and implements the business logic.
pub struct Application {
    state: State,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            state: State::default(),
        }
    }
}

impl Application {
    /// Initialize the theme and style when application starts up
    pub fn init_theme_and_style(&mut self, ctx: &egui::Context) {
        if !self.state.theme_initialized {
            self.state.theme_initialized = true;
            theme::set_custom_theme(ctx, &self.state.active_theme);
            theme::set_custom_style(ctx, &self.state.layout);
        }
    }

    /// Update the theme if the user selects a different theme
    pub fn update_theme(&mut self, ctx: &egui::Context, theme: theme::Catppuccin) {
        self.state.active_theme = theme;
        theme::set_custom_theme(ctx, &self.state.active_theme);
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.init_theme_and_style(ctx);

        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui_horizontal| {
                let name_label = ui_horizontal.label("Your name: ");

                ui_horizontal
                    .text_edit_singleline(&mut self.state.name)
                    .labelled_by(name_label.id);
            });

            ui.add(egui::Slider::new(&mut self.state.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.state.age += 1;
            }

            ui.label(format!(
                "Hello '{}', age {}",
                self.state.name, self.state.age
            ));

            if ui.button("Change Theme").clicked() {
                let new_theme: theme::Catppuccin = theme::LATTE;
                self.update_theme(ctx, new_theme);
            }
        });
    }
}
