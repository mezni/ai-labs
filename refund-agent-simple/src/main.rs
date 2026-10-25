mod ai;
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
    /// Output as JSON
    #[arg(long)]
    json: bool,
    /// Dry run - validate only, do not process
    #[arg(long)]
    dry_run: bool,
    /// AI review using OpenAI gpt-6-luna
    #[arg(long)]
    ai_review: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let args = Args::parse();

    let refund = Refund {
        order_id: args.order_id.trim().to_string(),
        amount: args.amount,
        reason: args.reason.trim().to_string(),
    };

    policy::validate(&refund)?;

    if args.ai_review {
        let review = ai::review_refund(&refund.order_id, refund.amount, &refund.reason).await?;
        if args.json {
            let out = serde_json::json!({
                "ai_review": true,
                "model": "gpt-6-luna",
                "order_id": refund.order_id,
                "amount": refund.amount,
                "reason": refund.reason,
                "review": review
            });
            println!("{}", serde_json::to_string_pretty(&out)?);
        } else {
            println!("[ai-review:gpt-6-luna] {}", review);
        }
        return Ok(());
    }

    if args.dry_run {
        if args.json {
            let dry = serde_json::json!({
                "dry_run": true,
                "status": "valid",
                "order_id": refund.order_id,
                "amount": refund.amount,
                "reason": refund.reason,
                "message": "Validation passed"
            });
            println!("{}", serde_json::to_string_pretty(&dry)?);
        } else {
            println!(
                "[dry-run] valid - order_id={} amount={:.2} reason={}",
                refund.order_id, refund.amount, refund.reason
            );
        }
        return Ok(());
    }

    let result = processor::process_refund(&refund);

    if args.json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!(
            "[{}] order_id={} amount={:.2} reason={} - {}",
            result.status, result.order_id, result.amount, result.reason, result.message
        );
    }
    Ok(())
}
