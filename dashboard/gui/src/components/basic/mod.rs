//! Holds the basic components used in the dashboard.

mod display_string;
mod button_group;
mod modal;
mod switch;

pub use display_string::DisplayString;
pub use button_group::button_group_clicked;
pub use modal::{modal, ModalResponse};