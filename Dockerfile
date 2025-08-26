# Multi-stage build for Axum Base with Tailwind CSS
FROM rust:1.89-bookworm as rust-builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release && rm src/main.rs

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Build the application
RUN cargo build --release

# Tailwind CSS builder stage
FROM node:24-alpine as tailwind-builder

# Install Tailwind CSS standalone CLI
RUN wget -O tailwindcss https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 \
    && chmod +x tailwindcss \
    && mv tailwindcss /usr/local/bin/

WORKDIR /app

# Copy Tailwind source and templates for class detection
COPY input.css ./
COPY templates ./templates

# Build production CSS with minification
RUN tailwindcss -i ./input.css -o ./static/style.css --minify

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app user for security
RUN groupadd -r appuser && useradd -r -g appuser appuser

WORKDIR /app

# Copy the built application from rust-builder
COPY --from=rust-builder /app/target/release/axum-base ./

# Copy built CSS from tailwind-builder
COPY --from=tailwind-builder /app/static ./static

# Copy templates
COPY templates ./templates

# Copy migration files
COPY migrations ./migrations

# Change ownership to app user
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose the port
EXPOSE 3093

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3093/health || exit 1

# Set environment variables
ENV RUST_LOG=info
ENV PORT=3093
ENV HOST=0.0.0.0

# Run the application
CMD ["./axum-base"]
