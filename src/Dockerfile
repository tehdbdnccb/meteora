FROM rust:1.80-bookworm AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends g++ \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /app/target/release/aegis-engine /usr/local/bin/aegis-engine

CMD ["/usr/local/bin/aegis-engine"]
