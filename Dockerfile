FROM rust:bookworm AS build

WORKDIR /src
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        curl \
        git \
        ripgrep \
    && rm -rf /var/lib/apt/lists/*
RUN rustup component add rustfmt clippy
COPY . .
RUN cargo build --release -p lkjagent-cli

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        busybox \
        ca-certificates \
        curl \
        git \
        ripgrep \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --home-dir /home/agent --shell /usr/sbin/nologin agent \
    && mkdir -p /data/workspace /usr/local/share/lkjagent/skills \
    && chown -R agent:agent /data \
    && printf '%s\n' \
        '#!/bin/sh' \
        'set -eu' \
        'mkdir -p /data/workspace' \
        'chown -R agent:agent /data' \
        'case "${1:-}" in' \
        '  ""|run|send|status|log|console|memory|skills)' \
        '    set -- /usr/local/bin/lkjagent "$@"' \
        '    ;;' \
        'esac' \
        'exec setpriv --reuid=1000 --regid=1000 --init-groups -- "$@"' \
        > /usr/local/bin/lkjagent-entrypoint \
    && chmod +x /usr/local/bin/lkjagent-entrypoint

COPY --from=build /src/target/release/lkjagent /usr/local/bin/lkjagent
COPY --from=build /src/crates/lkjagent-skills/seeds \
    /usr/local/share/lkjagent/skills

WORKDIR /data/workspace
ENTRYPOINT ["/usr/local/bin/lkjagent-entrypoint"]
