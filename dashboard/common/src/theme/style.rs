//! Custom Style implementation

use super::CustomStyle;
use eframe::egui;

/// Custom style implementation for the application layout
pub struct Layout {
    /// Default width of a egui Slider
    pub slider_width: f32,
    /// Horizontal and vertical spacing between widgets.
    pub item_spacing: egui::Vec2,
    /// Padding around buttons
    pub button_padding: egui::Vec2,
    /// Size of text
    pub text_size: f32,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            slider_width: 200_f32,
            item_spacing: egui::vec2(15.0, 15.0),
            button_padding: egui::vec2(10.0, 10.0),
            text_size: 14.0,
        }
    }
}

impl CustomStyle for Layout {
    fn style(&self, old_style: egui::Style) -> egui::Style {
        egui::Style {
            spacing: egui::style::Spacing {
                slider_width: self.slider_width,
                item_spacing: self.item_spacing,
                button_padding: self.button_padding,
                ..old_style.spacing
            },
            text_styles: {
                let mut text_styles = old_style.text_styles.clone();
                text_styles.get_mut(&egui::TextStyle::Body).unwrap().size = self.text_size;
                text_styles
            },
            ..old_style
        }
    }
}
