FROM rust:1.94.1 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/crabbo /usr/local/bin/
CMD ["/usr/local/bin/crabbo"]
