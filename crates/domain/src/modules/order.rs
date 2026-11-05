#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order;

impl Default for Order {
    fn default() -> Self {
        Self::new()
    }
}

impl Order {
    pub fn new() -> Self {
        Self
    }
}
