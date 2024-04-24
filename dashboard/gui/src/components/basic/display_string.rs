//! Custom display string for basic components

/// [`DisplayString`] is a custom display string for basic components
#[derive(Clone)]
pub struct DisplayString {
    /// The id of the component
    pub id: String,
    /// The display string of the component
    pub display: String,
}

impl From<(String, String)> for DisplayString {
    fn from((id, display): (String, String)) -> Self {
        Self { id, display }
    }
}

impl std::ops::Deref for DisplayString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}
