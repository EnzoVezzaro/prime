# Prime - Multi-stage Dockerfile
# Build stage
FROM rust:1.78-slim as builder

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libclang-dev \
    cmake \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY prime-core/Cargo.toml prime-core/
COPY prime-parser/Cargo.toml prime-parser/
COPY prime-index/Cargo.toml prime-index/
COPY prime-query/Cargo.toml prime-query/
COPY prime-cli/Cargo.toml prime-cli/
COPY prime-bench/Cargo.toml prime-bench/

# Pre-fetch dependencies
RUN cargo fetch --locked

# Copy source code
COPY prime-core/src ./prime-core/src/
COPY prime-parser/src ./prime-parser/src/
COPY prime-index/src ./prime-index/src/
COPY prime-query/src ./prime-query/src/
COPY prime-cli/src ./prime-cli/src/
COPY prime-bench/src ./prime-bench/src/

# Build release binary
RUN cargo build --release --workspace --bin prime

# Runtime stage
FROM debian:bookworm-slim as runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libclang1 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -r -s /bin/bash -m prime

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/prime /usr/local/bin/prime

# Create data directory
RUN mkdir -p /data/prime && chown -R prime:prime /data/prime

USER prime

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the binary
ENTRYPOINT ["prime"]
CMD ["serve"]