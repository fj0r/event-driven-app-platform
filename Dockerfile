ARG BASEIMAGE
FROM ${BASEIMAGE} as build
FROM scratch
COPY --from=build /app/target/dx/faucet_dx/release/web/public/ /app
