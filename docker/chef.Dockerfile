FROM rust:1 AS chef 
# We only pay the installation cost once, 
# it will be cached from the second build onwards
RUN set -eux \
  ; cargo install cargo-chef \
  ; cargo install dioxus-cli@0.7.0-rc.0 --force \
  ; apt update \
  ; apt-get install -y --no-install-recommends \
        ripgrep cmake \
  ;
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin gateway
RUN set -eux \
  ; cd ui \
  ; cat index.html | rg --passthru 'data-host=".+"' -r '' > index.html \
  ; dx build --platform web --release \
  \
  ; mkdir -p assets \
  ; cd assets \
  ; wget https://cdnjs.cloudflare.com/ajax/libs/mermaid/11.5.0/mermaid.min.js \
  ; wget https://cdnjs.cloudflare.com/ajax/libs/apexcharts/4.5.0/apexcharts.min.js \
  ; wget https://cdnjs.cloudflare.com/ajax/libs/apexcharts/4.5.0/apexcharts.min.css \
  ;

# We do not need the Rust toolchain to run the binary!
FROM debian:stable-slim AS runtime
WORKDIR /app

COPY --from=builder /app/target/release/gateway /app/gateway
COPY gateway.toml /app
COPY manifest /app/manifest

COPY --from=builder /app/target/dx/ui/release/web/public/ /app
COPY --from=builder /app/ui/assets /app/assets
