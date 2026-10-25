use anyhow::{Result, bail};
use clap::Parser;

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

    if args.order_id.trim().is_empty() {
        bail!("order_id cannot be empty");
    }
    if args.amount <= 0.0 {
        bail!("amount must be greater than 0");
    }
    if args.reason.trim().is_empty() {
        bail!("reason cannot be empty");
    }

    println!(
        "Processing refund: order_id={}, amount={:.2}, reason={}",
        args.order_id.trim(),
        args.amount,
        args.reason.trim()
    );
    Ok(())
}
