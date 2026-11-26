# syntax=docker/dockerfile:1
FROM rust:1-bookworm AS builder
WORKDIR /app
ENV SQLX_OFFLINE=true

# Cache dependencies in a separate layer.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release --bin backend \
    && rm -rf src

COPY .sqlx ./.sqlx
COPY src ./src
RUN touch src/main.rs && cargo build --release --bin backend

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/backend /usr/local/bin/backend
ENV BIND_ADDR=0.0.0.0:8080
EXPOSE 8080
CMD ["backend"]
