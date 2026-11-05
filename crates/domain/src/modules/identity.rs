#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity;

impl Default for Identity {
    fn default() -> Self {
        Self::new()
    }
}

impl Identity {
    pub fn new() -> Self {
        Self
    }
}
