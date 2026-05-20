FROM rust:1.78-slim-bookworm AS builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
RUN cargo build --release --bin cortex && strip target/release/cortex

FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=builder /app/target/release/cortex /usr/local/bin/cortex
COPY cortex.toml /etc/cortex/cortex.toml
EXPOSE 8787
ENTRYPOINT ["cortex"]
CMD ["serve", "--port", "8787"]
