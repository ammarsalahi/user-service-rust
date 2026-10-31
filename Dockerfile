# ==========================================
# Builder
# ==========================================
FROM rust:1.98 AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
        ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Configure ITO Rust mirror
# RUN mkdir -p /usr/local/cargo

# RUN cat > /usr/local/cargo/config.toml <<'EOF'
# [source.crates-io]
# replace-with = "ito-mirror"

# [source.ito-mirror]
# registry = "sparse+https://archive.ito.gov.ir/rust/"
# EOF

# Copy dependency manifests
COPY Cargo.toml Cargo.lock ./

# Cache dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy complete project
COPY . .

# Build application
# `touch` is required: COPY preserves the host mtimes of the sources, which can be
# older than the cached artifacts left by the dummy build above. Cargo compares
# mtimes, would consider the crate fresh, and silently keep the stub binary.
RUN find src -name '*.rs' -exec touch {} + && \
    rm -f target/release/user-service-rust && \
    cargo build --release


# ==========================================
# Runtime
# ==========================================
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy compiled binary
COPY --from=builder /app/target/release/user-service-rust ./user-service-rust

EXPOSE 3000

ENV RUST_LOG=info

CMD ["./user-service-rust"]
