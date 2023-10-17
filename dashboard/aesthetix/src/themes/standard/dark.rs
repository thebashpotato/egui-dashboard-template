//! A Standard Dark theme that should look good on any platform.
//! This is basically libadwaita from the Gnome project.

use crate::themes::traits::Aesthetix;
use egui;

/// A standard dark theme.
/// https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1-latest/named-colors.html
#[derive(Debug, Clone, Copy)]
pub struct StandardDark;

impl Aesthetix for StandardDark {
    fn name(&self) -> &str {
        "Standard Dark"
    }

    fn primary_accent_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(98, 160, 234)
    }

    fn secondary_accent_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(53, 132, 228)
    }

    fn bg_primary_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(30, 30, 30)
    }

    fn bg_secondary_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(48, 48, 48)
    }

    fn bg_triage_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(40, 40, 40)
    }

    fn bg_auxiliary_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(72, 72, 72)
    }

    fn hyperlink_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(109, 109, 109)
    }

    fn fg_primary_text_color_visuals(&self) -> Option<egui::Color32> {
        Some(egui::Color32::from_rgb(255, 255, 255))
    }

    fn fg_success_text_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(38, 162, 105)
    }

    fn fg_warn_text_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(205, 147, 9)
    }

    fn fg_error_text_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(192, 28, 40)
    }

    fn dark_mode_visuals(&self) -> bool {
        true
    }

    fn margin_style(&self) -> f32 {
        10.0
    }

    fn button_padding(&self) -> egui::Vec2 {
        egui::Vec2 { x: 10.0, y: 8.0 }
    }

    fn item_spacing_style(&self) -> f32 {
        15.0
    }

    fn scroll_bar_width_style(&self) -> f32 {
        12.0
    }

    fn rounding_visuals(&self) -> f32 {
        8.0
    }
}
