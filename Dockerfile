# syntax=docker/dockerfile:1

FROM rust:alpine as build
WORKDIR /app

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
COPY --link --from=build /symbolab_rs /symbolab_rs
ENTRYPOINT /symbolab_rs