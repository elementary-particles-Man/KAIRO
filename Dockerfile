# ===========================
# 📄 Dockerfile
# ===========================

# Build stage
FROM rust:1.77 AS builder
WORKDIR /usr/src/kairo
COPY . .

# Build both mesh and coordination node binaries
RUN cargo build --release -p mesh-node \
    && cargo build --release --manifest-path AI-TCP/core/kairo_coord_node/Cargo.toml

# Runtime stage
FROM debian:bullseye-slim
WORKDIR /app

# Copy mesh-node binary
COPY --from=builder /usr/src/kairo/target/release/mesh-node /usr/local/bin/mesh-node

# Copy coordination node binary
COPY --from=builder /usr/src/kairo/target/release/kairo_coord_node /usr/local/bin/kairo_coord_node

# Copy shared library if needed
COPY --from=builder /usr/src/kairo/target/release/librust_core.so /usr/local/lib/librust_core.so

# Default entrypoint (can be overridden)
CMD ["mesh-node"]
