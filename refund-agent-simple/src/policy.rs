use anyhow::{Result, bail};

pub fn validate(refund_req: &crate::models::Refund) -> Result<()> {
    if refund_req.order_id.trim().is_empty() {
        bail!("order_id cannot be empty");
    }
    if refund_req.amount <= 0.0 {
        bail!("amount must be greater than 0");
    }
    if refund_req.reason.trim().is_empty() {
        bail!("reason cannot be empty");
    }
    if refund_req.amount > 1000.0 {
        bail!("amount exceeds max refund of 1000.00");
    }
    if !is_allowed_reason(&refund_req.reason) {
        bail!("reason must be one of: damaged, late_delivery, wrong_item");
    }
    Ok(())
}

fn is_allowed_reason(r: &str) -> bool {
    matches!(r, "damaged" | "late_delivery" | "wrong_item")
}
