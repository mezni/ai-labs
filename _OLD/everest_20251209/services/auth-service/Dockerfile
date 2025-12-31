FROM rust:1.90 as builder

WORKDIR /app

# Copy full workspace first
COPY . .

# Enable SQLx offline mode
ENV SQLX_OFFLINE=true

# Build dependencies (cached)
RUN cargo build --release --locked --workspace --bins

# Copy final binary
FROM debian:stable-slim
WORKDIR /app

COPY --from=builder /app/target/release/auth-service /app/auth-service
COPY .env /app/.env

EXPOSE 3000
CMD ["./auth-service"]
