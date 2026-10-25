
use refund_agent::models::{
    Customer,
    Order,
    OrderStatus,
    RefundDecision,
    RefundRequest,
    RefundStatus,
};

fn main() {
    let customer = Customer {
        id: "CUST-001".to_string(),
        name: "Alex Morgan".to_string(),
    };

    let order = Order {
        id: "ORD-1001".to_string(),
        customer_id: customer.id.clone(),
        total_amount_cents: 4_999,
        status: OrderStatus::Delivered,
    };

    let request = RefundRequest {
        id: "REF-001".to_string(),
        customer_id: customer.id.clone(),
        order_id: order.id.clone(),
        reason: "The item arrived damaged".to_string(),
    };

    let decision = RefundDecision::RequiresHumanReview {
        reason: "Damage evidence must be reviewed".to_string(),
    };

    let status = RefundStatus::UnderReview;

    println!("Customer: {customer:#?}");
    println!("Order: {order:#?}");
    println!("Refund request: {request:#?}");
    println!("Refund decision: {decision:#?}");
    println!("Refund status: {status:#?}");
}
