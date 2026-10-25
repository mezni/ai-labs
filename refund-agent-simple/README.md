# refund-agent-simple

Simple refund agent in Rust. Built step by step.

## Usage

```bash
cargo run -- --order-id ORD-123 --amount 15.00 --reason damaged
cargo run -- --order-id ORD-123 --amount 15.00 --reason damaged --dry-run
cargo run -- --order-id ORD-123 --amount 15.00 --reason damaged --json
```

## Policy
- Amount > 0 and <= 1000
- Reasons: `damaged`, `late_delivery`, `wrong_item`
- Order ID and reason non-empty
