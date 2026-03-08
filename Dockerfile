# Multi-stage build for Axum Base with Tailwind CSS
FROM rust:1.94-trixie AS rust-builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && ARCH=$(uname -m) \
    && if [ "$ARCH" = "x86_64" ]; then TAILWIND_ARCH="x64"; \
       elif [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "arm64" ]; then TAILWIND_ARCH="arm64"; \
       else echo "Unsupported architecture: $ARCH" && exit 1; fi \
    && curl -LO "https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-$TAILWIND_ARCH" \
    && chmod +x "tailwindcss-linux-$TAILWIND_ARCH" \
    && mv "tailwindcss-linux-$TAILWIND_ARCH" /usr/local/bin/tailwindcss \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy all files for building
# (Simplified: removed dummy caching to resolve issues with multiple binaries)
COPY . .

# Set environment variables for SQLx offline mode
ENV SQLX_OFFLINE=true

# Build the application in release mode
RUN cargo build --release

# Runtime stage: Use Google Distroless for security and minimalism
FROM gcr.io/distroless/cc-debian13

# Set working directory
WORKDIR /app

# Copy the built application from rust-builder
COPY --from=rust-builder /app/target/release/axum-base ./

# Copy built CSS and static assets from rust-builder
COPY --from=rust-builder /app/static ./static

# Copy templates
COPY --from=rust-builder /app/templates ./templates

# Copy migration files
COPY --from=rust-builder /app/migrations ./migrations

# Expose the port
EXPOSE 3093

# Set environment variables
ENV RUST_LOG=info
ENV PORT=3093
ENV HOST=0.0.0.0

# Run the application
CMD ["./axum-base"]
