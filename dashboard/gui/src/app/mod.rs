//! Application state and business logic.
mod state;

use eframe::egui;


use state::State;

/// Holds application state and implements the business logic.
#[derive(Debug)]
pub struct Application {
    state: State,
}

impl Application {
    /// Create a new application    
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: State::new(),
        }
    }

    /// Initialize the custom theme/styles for egui
    fn init_theme(&mut self, ctx: &egui::Context) {
        if !self.state.is_theme_initialized {
            println!("Setting theme: {}", self.state.active_theme.name());
            ctx.set_style(self.state.active_theme.custom_style());
            self.state.is_theme_initialized = true;
        }
    }
}

impl eframe::App for Application {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.init_theme(ctx);

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
        });
    }
}
