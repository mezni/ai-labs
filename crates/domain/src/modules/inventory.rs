#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inventory;

impl Default for Inventory {
    fn default() -> Self {
        Self::new()
    }
}

impl Inventory {
    pub fn new() -> Self {
        Self
    }
}
