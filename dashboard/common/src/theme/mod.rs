//! Custom themes for [egui](egui).
//!
//! To use, call [`set_custom_theme`](crate::set_theme) with the egui context
//! and a [`CustomTheme`](crate::traits::CustomTheme).

mod colorscheme;
mod style;
mod traits;

pub use colorscheme::{Catppuccin, FRAPPE, LATTE, MACCHIATO, MOCHA};
pub use style::Layout;
pub use traits::{CustomStyle, CustomTheme};

use eframe::egui;

/// Apply the given customized theme to a [`Context`](egui::Context).
pub fn set_custom_theme(ctx: &egui::Context, custom_theme: &impl CustomTheme) {
    let old_visuals = ctx.style().visuals.clone();
    ctx.set_visuals(custom_theme.visuals(old_visuals));
}

/// Apply the given customized application layout style to a [`Style`](egui::Style).
/// Sets the style of buttons, input fields, etc.
pub fn set_custom_style(ctx: &egui::Context, layout: &impl CustomStyle) {
    let old_style = (*ctx.style()).clone();
    ctx.set_style(layout.style(old_style));
}
