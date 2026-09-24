FROM rust:latest AS base
COPY ./booth2rss /build/booth2rss
COPY ./booth2rss_web /build/booth2rss_web

#FROM base AS build
#WORKDIR "/build/booth2rss_web"
#RUN cargo build

FROM base AS test
WORKDIR "/build"
RUN cargo test --workspace

FROM test AS build-release
WORKDIR "/build/booth2rss_web"
RUN cargo build --release
RUN mkdir -p "/app"
RUN cp "target/release/booth2rss_web" "/app/booth2rss_web"

# FROM build-release AS cleanup
# WORKDIR "/build/booth2rss_web"
# RUN cargo clean

FROM build-release AS run
EXPOSE 8080
ENTRYPOINT ["/app/booth2rss_web"]
