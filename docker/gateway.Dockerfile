ARG BASEIMAGE=ghcr.io/fj0r/edap:build.gateway
FROM ${BASEIMAGE} AS build
FROM ghcr.io/fj0r/edap:ui AS assets
FROM debian:stable-slim
RUN apt update \
 && apt-get install -y --no-install-recommends libssl3 \
 && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/gateway /app/gateway
COPY --from=assets /app /app/static
COPY gateway.toml /app
COPY manifest /app/manifest
