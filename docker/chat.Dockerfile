FROM rust:1 AS chef
# We only pay the installation cost once,
# it will be cached from the second build onwards
RUN set -eux \
  ; cargo install cargo-chef \
  ; cargo install dioxus-cli@0.7.0-rc.1 --force \
  ; apt update \
  ; apt-get install -y --no-install-recommends \
        ripgrep cmake \
  ;

FROM chef AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS cache
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

FROM cache AS builder
WORKDIR /app
# Build application
COPY . .
RUN cargo build --release --bin chat


FROM debian:stable-slim AS runtime
WORKDIR /app

COPY --from=builder /app/target/release/chat /app/chat
COPY chat.toml /app
COPY manifest /app/manifest
