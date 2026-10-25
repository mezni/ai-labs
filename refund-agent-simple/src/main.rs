use anyhow::Result;
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
    println!(
        "Processing refund: order_id={}, amount={:.2}, reason={}",
        args.order_id, args.amount, args.reason
    );
    Ok(())
}
