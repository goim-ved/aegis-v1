FROM rust:slim-bookworm as builder

WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/proxy_fintech_v1 /app/proxy_fintech_v1
COPY --from=builder /app/migrations /app/migrations

CMD ["./proxy_fintech_v1"]
