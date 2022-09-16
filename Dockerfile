# syntax=docker/dockerfile:1

FROM rust:alpine as build
ARG RUSTFLAGS="--cfg tokio_unstable -C target-cpu=native"

WORKDIR /app

RUN apk add musl-dev
COPY --link . .
RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/app/target cargo build --release
RUN --mount=type=cache,target=/app/target cp target/release/symbolab_rs /
# COPY --link src .


# RUN --mount=type=cache,target=/usr/local/cargo/registry <<EOF
#   set -e
#   # update timestamps to force a new build
#   touch src/main.rs
#   cargo build --release
# EOF

FROM alpine AS runner
ENV RUST_LOG="symbolab_rs=debug,tower_http=warn"

COPY --link --from=build /symbolab_rs /symbolab_rs
ENTRYPOINT /symbolab_rs