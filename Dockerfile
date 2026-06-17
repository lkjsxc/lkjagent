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
    && mkdir -p /data /workspace /usr/local/share/lkjagent/skills \
    && chown agent:agent /data /workspace /usr/local/share/lkjagent/skills

COPY --from=build /src/target/release/lkjagent /usr/local/bin/lkjagent
COPY --from=build --chown=agent:agent /src/crates/lkjagent-skills/seeds \
    /usr/local/share/lkjagent/skills

USER agent
WORKDIR /workspace
ENTRYPOINT ["/usr/local/bin/lkjagent"]
