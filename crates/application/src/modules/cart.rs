#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cart;

impl Default for Cart {
    fn default() -> Self {
        Self::new()
    }
}

impl Cart {
    pub fn new() -> Self {
        Self
    }
}