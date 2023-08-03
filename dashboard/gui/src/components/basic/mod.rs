//! Holds the basic components used in the dashboard.

mod button_group;
mod display_string;
mod modal;
mod switch;

pub use button_group::button_group_clicked;
pub use display_string::DisplayString;
pub use modal::{modal, ModalResponse};
