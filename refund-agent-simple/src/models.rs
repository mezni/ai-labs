#[derive(Debug)]
pub struct Refund {
    pub order_id: String,
    pub amount: f64,
    pub reason: String,
}

#[derive(Debug)]
pub struct RefundResult {
    pub status: String,
    pub order_id: String,
    pub amount: f64,
    pub reason: String,
    pub message: String,
}
