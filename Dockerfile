###########################
#          Build          #
###########################
FROM rust:slim AS base
COPY . /build/booth2rss

FROM base AS test
WORKDIR "/build/booth2rss"
ARG RUSTFLAGS
RUN cargo test --release

FROM test AS build
WORKDIR "/build/booth2rss"
ARG RUSTFLAGS
RUN cargo build --release --bin booth2rss_web
RUN mkdir -p "/app"
RUN cp "target/release/booth2rss_web" "/app/booth2rss_web"


###########################
#         Runtime         #
###########################
FROM alpine:latest
COPY --from=build "/build/booth2rss/target/release/booth2rss_web" "/app/booth2rss_web"
COPY ./docker-entry.sh /app/docker-entry.sh
RUN touch /app/config.json
RUN chown 1001 /app/config.json

ENV RUST_LOG="Info" 
ENV LISTEN_ADDRESS="0.0.0.0:8080"
ENV CURRENCY_FALLBACK="JPY"
ENV STORE_CACHE_MINUTES="15"
ENV STORE_CACHE_SIZE="50"
ENV ALLOW_CURRENCY_CONVERSION="true"
ENV CURRENCY_CACHE_MINUTES="120"
ENV CURRENCY_CACHE_SIZE="10"

USER 1001:1001
EXPOSE 8080
WORKDIR "/app/"
ENTRYPOINT ["/app/docker-entry.sh"]
