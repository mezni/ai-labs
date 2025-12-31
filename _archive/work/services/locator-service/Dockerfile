# Use Rust version that matches your local
FROM rust:1.90-alpine as builder

# Install build dependencies including curl
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig \
    curl

WORKDIR /app

# Copy entire project
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache openssl ca-certificates tzdata libgcc

# Create non-root user
RUN addgroup -S appgroup && adduser -S appuser -G appgroup

WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/locator-service /app/locator-service

# Change ownership to appuser
RUN chown -R appuser:appgroup /app

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 5100

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:5100/api/v1/health || exit 1

# Run the application
CMD ["./locator-service"]