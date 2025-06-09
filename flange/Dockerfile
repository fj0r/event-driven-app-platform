ARG BASEIMAGE=ghcr.io/fj0r/faucet_dx:latest
FROM ${BASEIMAGE} AS build
FROM ghcr.io/fj0r/faucet_dx:lastest AS assets
FROM debian:stable-slim
RUN apt update \
 && apt-get install -y --no-install-recommends libssl3 \
 && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/flange /app/flange
COPY --from=assets /app /app/static
COPY config.toml /app
COPY assets /app/assets
