FROM rust:1.94.1-alpine AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:3.19
RUN apk add --no-cache ca-certificates openssl
COPY --from=builder /app/target/release/crabbo /usr/local/bin/
CMD ["/usr/local/bin/crabbo"]
