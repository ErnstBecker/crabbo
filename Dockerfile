FROM rust:1.94.1 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM ubuntu:24.04
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/crabbo /usr/local/bin/
CMD ["/usr/local/bin/crabbo"]
