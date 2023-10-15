//! Carl is a theme for kde plasma
//! https://store.kde.org/p/1338881/

use crate::themes::traits::Aesthetix;
use egui;

/// A Dark theme based on the Carl Kde desktop theme
#[derive(Debug)]
pub struct CarlDark;

impl Aesthetix for CarlDark {
    fn name(&self) -> &str {
        "Carl Dark"
    }

    fn primary_accent_color_visuals(&self) -> egui::Color32 {
        // Color #5
        egui::Color32::from_rgb(56, 114, 238)
    }

    fn secondary_accent_color_visuals(&self) -> egui::Color32 {
        // Color #7
        egui::Color32::from_rgb(87, 188, 187)
    }

    fn bg_primary_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(17, 18, 22)
    }

    fn bg_secondary_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(25, 27, 33)
    }

    fn bg_triage_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(12, 12, 15)
    }

    fn bg_code_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(72, 72, 72)
    }

    fn hyperlink_color_visuals(&self) -> egui::Color32 {
        // Intense Color #1
        egui::Color32::from_rgb(109, 109, 109)
    }

    fn fg_primary_text_color_visuals(&self) -> Option<egui::Color32> {
        Some(egui::Color32::from_rgb(240, 218, 218))
    }

    fn fg_warn_text_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(176, 49, 180)
    }

    fn fg_error_text_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(241, 52, 96)
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
