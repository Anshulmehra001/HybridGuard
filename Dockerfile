# HybridGuard Docker Container
# Multi-stage build for smaller image size

# Stage 1: Build
FROM rust:1.75-slim as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    liboqs-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy source code
COPY . .

# Build release binary
RUN cargo build --release

# Stage 2: Runtime
FROM ubuntu:22.04

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    liboqs0 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/hybridguard /usr/local/bin/

# Create data directory
RUN mkdir -p /data

# Set working directory
WORKDIR /data

# Default command
CMD ["hybridguard", "status"]
