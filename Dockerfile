FROM rust as build

WORKDIR /app
COPY . .
RUN cargo install dioxus-cli \
 && dx build --platform web --release

FROM scratch
COPY --from=build /app/target/dx/faucet_dx/release/web/public/ /app
