# Build stage
FROM rust:1.83-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy src to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "fn main() {}" > src/lib.rs

# Build dependencies
RUN cargo build --release

# Remove dummy source
RUN rm -rf src

# Copy actual source code
COPY . .

# Build application
# We need to touch the main file to force a rebuild of the application code
RUN touch src/lib.rs && touch src/main.rs
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy binary from builder - binary name matches package name "artist-portfolio"
COPY --from=builder /app/target/release/artist-portfolio /app/artist-portfolio

# Copy static assets and templates
COPY --from=builder /app/static /app/static
COPY --from=builder /app/templates /app/templates

# Expose port (Render sets PORT env var, but we expose 8000 as default)
EXPOSE 8000

# Run the application
CMD ["./artist-portfolio"]
