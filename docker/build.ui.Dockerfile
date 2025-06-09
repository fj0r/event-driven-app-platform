ARG BASEIMAGE=rust
FROM ${BASEIMAGE}

WORKDIR /app
COPY ui .

RUN apt update \
 && apt-get install -y --no-install-recommends ripgrep \
 && cargo install dioxus-cli \
 && cat index.html | rg --passthru 'data-host=".+"' -r '' > index.html \
 && dx build --platform web --release
