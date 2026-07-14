# syntax=docker/dockerfile:1.7
FROM rust:bookworm AS build

WORKDIR /src
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        curl \
        git \
        ripgrep \
    && rm -rf /var/lib/apt/lists/*
RUN rustup component add rustfmt clippy
COPY Cargo.toml Cargo.lock README.md AGENTS.md ./
COPY .gitignore .dockerignore ./
COPY .cargo ./.cargo
COPY .github/workflows/verify.yml ./.github/workflows/verify.yml
COPY crates/lkjagent-app ./crates/lkjagent-app
COPY crates/lkjagent-core ./crates/lkjagent-core
COPY crates/lkjagent-effects ./crates/lkjagent-effects
COPY crates/lkjagent-llm ./crates/lkjagent-llm
COPY crates/lkjagent-store ./crates/lkjagent-store
COPY crates/lkjagent-xtask ./crates/lkjagent-xtask
COPY docs ./docs
COPY evaluation ./evaluation
COPY config/lkjagent.example.json ./config/lkjagent.example.json
COPY Dockerfile docker-compose.yml ./
RUN git init \
    && git config user.name container \
    && git config user.email container@invalid \
    && git add . \
    && git commit -m "container source snapshot" \
        -m "Tested: copied build context is tracked" \
        -m "Not-tested: external repository history is unavailable" \
    && git commit --allow-empty -m "container test boundary" \
        -m "Tested: parent source snapshot is reachable" \
        -m "Not-tested: external repository history is unavailable"
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/src/target \
    cargo build --locked --release -p lkjagent-app \
    && cp /src/target/release/lkjagent /tmp/lkjagent

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        busybox \
        ca-certificates \
        curl \
        git \
        ripgrep \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --uid 1000 --create-home --home-dir /home/agent --shell /usr/sbin/nologin agent \
    && mkdir -p /data /workspace /usr/local/share/lkjagent/skills \
    && chown agent:agent /data /workspace \
    && printf '%s\n' \
        '#!/bin/sh' \
        'set -eu' \
        'chown agent:agent /data /workspace' \
        'cd /workspace' \
        'case "${1:-}" in' \
        '  ""|run|send|status|tui|doctor|help)' \
        '    set -- /usr/local/bin/lkjagent --data /data "$@"' \
        '    ;;' \
        'esac' \
        'exec setpriv --reuid=1000 --regid=1000 --init-groups -- "$@"' \
        > /usr/local/bin/lkjagent-entrypoint \
    && chmod +x /usr/local/bin/lkjagent-entrypoint

COPY --from=build /tmp/lkjagent /usr/local/bin/lkjagent
COPY --from=build /src/config/lkjagent.example.json /usr/local/share/lkjagent/lkjagent.example.json

WORKDIR /
ENTRYPOINT ["/usr/local/bin/lkjagent-entrypoint"]
