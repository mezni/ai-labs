mod models;
mod policy;
mod processor;

use anyhow::Result;
use clap::Parser;
use models::Refund;

#[derive(Parser, Debug)]
#[command(name = "refund-agent-simple")]
#[command(about = "Simple refund agent", long_about = None)]
struct Args {
    /// Order ID
    #[arg(long)]
    order_id: String,
    /// Amount to refund
    #[arg(long)]
    amount: f64,
    /// Reason for refund
    #[arg(long)]
    reason: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let refund = Refund {
        order_id: args.order_id.trim().to_string(),
        amount: args.amount,
        reason: args.reason.trim().to_string(),
    };

    policy::validate(&refund)?;
    let result = processor::process_refund(&refund);
    println!(
        "[{}] order_id={} amount={:.2} reason={} - {}",
        result.status, result.order_id, result.amount, result.reason, result.message
    );
    Ok(())
}
