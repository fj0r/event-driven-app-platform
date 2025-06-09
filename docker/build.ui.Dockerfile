ARG BASEIMAGE=rust
FROM ${BASEIMAGE}

WORKDIR /app
COPY ui .

RUN set -eux \
  ; apt update \
  ; apt-get install -y --no-install-recommends ripgrep \
  ; cargo install dioxus-cli \
  ; cat index.html | rg --passthru 'data-host=".+"' -r '' > index.html \
  ; dx build --platform web --release \
  \
  ; mkdir -p assets \
  ; cd assets \
  ; wget https://cdnjs.cloudflare.com/ajax/libs/mermaid/11.5.0/mermaid.min.js \
  ; wget https://cdnjs.cloudflare.com/ajax/libs/apexcharts/4.5.0/apexcharts.min.js \
  ; wget https://cdnjs.cloudflare.com/ajax/libs/apexcharts/4.5.0/apexcharts.min.css \
  ;
