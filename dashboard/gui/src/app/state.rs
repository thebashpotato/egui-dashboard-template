//! holds the state of the application

use dashboard_aesthetix::themes::{Aesthetix, CarlDark};

/// Application state
#[derive(Debug)]
pub struct State {
    /// dummy state
    pub name: String,
    /// dummy state
    pub age: u32,
    /// The active theme
    // TODO: Make this a list of themes, and allow the user to switch between them.
    // TODO: Switch to static dispatching instead of dynamic dispatching.
    pub active_theme: Box<dyn Aesthetix>,
    /// so we don't reload the theme every frame
    pub is_theme_initialized: bool,
}

impl State {
    /// Create a new state with the CarlDark theme
    #[must_use]
    pub fn new() -> Self {
        Self {
            name: "John Doe".to_owned(),
            age: 42,
            active_theme: Box::new(CarlDark),
            is_theme_initialized: false,
        }
    }
}
