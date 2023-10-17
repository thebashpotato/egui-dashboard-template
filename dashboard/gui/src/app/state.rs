//! holds the state of the application

use dashboard_aesthetix::themes::Aesthetix;

/// The different tabs of the application
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Tab {
    /// The home tab
    Home,
    /// The settings tab
    Settings,
    /// The logs tab
    Logs,
    /// The debug tab
    Debug,
    /// The about tab
    About,
}

/// Application state
#[derive(Debug)]
pub struct State {
    /// The currently selected tab
    pub active_tab: Tab,
    /// The active theme
    // TODO: Make this a list of themes, and allow the user to switch between them.
    // TODO: Switch to static dispatching instead of dynamic dispatching.
    pub active_theme: Box<dyn Aesthetix>,
}

impl State {
    /// Create a new state with the CarlDark theme
    #[must_use]
    pub fn new(theme: Box<dyn Aesthetix>) -> Self {
        Self {
            active_tab: Tab::Home,
            active_theme: theme,
        }
    }
}
