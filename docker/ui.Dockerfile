ARG BASEIMAGE
FROM ${BASEIMAGE} as build
FROM scratch
COPY --from=build /app/target/dx/ui/release/web/public/ /app
COPY --from=build /app/ui/assets /app/assets
