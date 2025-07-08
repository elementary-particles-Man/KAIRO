FROM rust:1.75 as builder
WORKDIR /usr/src/kairo
COPY . .
RUN cargo build --release --manifest-path AI-TCP/core/kairo_coord_node/Cargo.toml

FROM debian:buster-slim
COPY --from=builder /usr/src/kairo/target/release/kairo_coord_node /usr/local/bin/kairo_coord_node
ENTRYPOINT ["kairo_coord_node"]
