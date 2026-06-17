FROM rust:bookworm AS build

WORKDIR /src
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
    && mkdir -p /data /workspace \
    && chown agent:agent /data /workspace

COPY --from=build /src/target/release/lkjagent /usr/local/bin/lkjagent

USER agent
WORKDIR /workspace
ENTRYPOINT ["/usr/local/bin/lkjagent"]
