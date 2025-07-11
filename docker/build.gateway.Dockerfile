ARG BASEIMAGE=rust
FROM ${BASEIMAGE}

WORKDIR /app
COPY . .
RUN apt update \
 && apt-get install -y --no-install-recommends cmake \
 && cargo build --release --bin gateway
