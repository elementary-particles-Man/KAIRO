# Build stage
FROM rust:1.77 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p mesh-node

# Runtime stage
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/mesh-node /usr/local/bin/mesh-node
COPY --from=builder /app/target/release/librust_core.so /usr/local/lib/librust_core.so
CMD ["mesh-node"]
