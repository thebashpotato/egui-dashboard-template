//! Traits for common theme functionality.

use eframe::egui;

/// Trait for customizing the [`Visuals`](egui::Visuals) of a [`Context`](egui::Context).
pub trait CustomTheme {
    /// Get the visuals for the given [`Visuals`](egui::Visuals).
    fn visuals(&self, old_visuals: egui::Visuals) -> egui::Visuals;
}

/// Trait for customizing the [`Style`](egui::Style) of a [`Context`](egui::Context).
pub trait CustomStyle {
    /// Get the style for the given [`Style`](egui::Style).
    fn style(&self, old_style: egui::Style) -> egui::Style;
}
