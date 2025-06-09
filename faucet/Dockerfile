ARG BASEIMAGE
FROM ${BASEIMAGE} as build
FROM scratch
COPY --from=build /app/target/dx/faucet/release/web/public/ /app
