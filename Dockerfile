FROM rust:slim AS base
COPY ./booth2rss /build/booth2rss
COPY ./booth2rss_web /build/booth2rss_web
COPY ./docker-entry.sh /app/docker-entry.sh
RUN touch /app/config.json
RUN chown 1001 /app/config.json

#FROM base AS build
#WORKDIR "/build/booth2rss_web"
#RUN cargo build

FROM base AS test
WORKDIR "/build/booth2rss"
RUN cargo test

FROM test AS build-release
WORKDIR "/build/booth2rss_web"
ARG RUSTFLAGS
RUN cargo build --release
RUN mkdir -p "/app"
RUN cp "target/release/booth2rss_web" "/app/booth2rss_web"

# FROM build-release AS cleanup
# WORKDIR "/build/booth2rss_web"
# RUN cargo clean

FROM build-release AS run

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
