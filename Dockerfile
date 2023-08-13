FROM ghcr.io/pesca-dev/leptos-builder-musl AS builder

WORKDIR /work

COPY . .

RUN cargo leptos build --release

########################################
########################################
########################################

FROM scratch as app

ENV LEPTOS_OUTPUT_NAME=aoc_website
ENV LEPTOS_SITE_ROOT=site
ENV LEPTOS_SITE_PKG_DIR=pkg
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_RELOAD_PORT=3001

USER 10001

WORKDIR /app

COPY --chown=10001:10001 --from=builder /work/target/site/ ./site/
COPY --chown=10001:10001 --from=builder /work/target/server/release/aoc_website .
COPY --chown=10001:10001 --from=builder /work/Cargo.toml .

# depends on the port you choose
EXPOSE 3000

# must match your final server executable name
ENTRYPOINT ["/app/aoc_website"]
