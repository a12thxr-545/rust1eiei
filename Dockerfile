# ─────────────────────────────────────────
# Stage 1 — Build Rust binary
# ─────────────────────────────────────────
FROM rust:1.86-slim AS builder

# Install system deps needed for diesel (libpq) and aws-lc-rs (cmake, clang)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libpq-dev \
    cmake \
    clang \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy dependency manifests first from the server subdirectory
COPY server/Cargo.toml server/Cargo.lock ./

# Create a dummy main to pre-compile dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release 2>/dev/null || true
RUN rm -rf src

# Copy actual source code from the server subdirectory and build
COPY server/src ./src
COPY server/diesel.toml ./

# Touch main.rs so cargo rebuilds the final binary
RUN touch src/main.rs
RUN cargo build --release

# ─────────────────────────────────────────
# Stage 2 — Minimal runtime image
# ─────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies (libpq for postgres)
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from builder stage
COPY --from=builder /app/target/release/server ./server

# Copy Angular static files from the server subdirectory
COPY server/statics ./statics

# Port 80 for HTTP (Render will use PORT environment variable if needed)
EXPOSE 80

CMD ["./server"]
