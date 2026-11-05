#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Catalog;

impl Default for Catalog {
    fn default() -> Self {
        Self::new()
    }
}

impl Catalog {
    pub fn new() -> Self {
        Self
    }
}
