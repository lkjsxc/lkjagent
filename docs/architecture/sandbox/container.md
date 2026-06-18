# Container

## Purpose

The container the harness runs in: image contents, the user, the mounts,
the environment, supervision, and network posture. The container boundary
is the safety mechanism per
[../../decisions/container-first.md](../../decisions/container-first.md).

## Image

A distroless-leaning Debian-slim image holding exactly what the agent
needs:

- the lkjagent binary,
- busybox-class core utilities,
- git, curl, and ripgrep,
- the seed skill files under /usr/local/share/lkjagent/skills.

Nothing else is promised. The entrypoint prepares /data ownership, then runs
the requested command as the non-root user named agent. There is no docker
socket, and no host filesystem is visible beyond the mounted data directory.
Compose runs the agent with a read-only root filesystem, tmpfs at /tmp and
/home/agent, no-new-privileges, and NET_RAW removed.

## Mounts

| Mount | Kind | Holds |
| --- | --- | --- |
| /data | bind mount | store file, workspace, skill library, config; layout in [workspace.md](workspace.md) |

## Environment

| Variable | Meaning |
| --- | --- |
| LKJAGENT_ENDPOINT_URL | where the chat-completions endpoint is reached |
| LKJAGENT_MODEL | the model name sent on every request |
| LKJAGENT_API_KEY | optional; sent to the endpoint when set |
| LKJAGENT_ENDPOINT_TIMEOUT_SECONDS | optional endpoint request timeout |

The API key, when needed, arrives by environment variable only: never baked
into the image and never written to the store, per
[../memory/store.md](../memory/store.md).

## Supervision

The container supervisor restarts the daemon on nonzero exit; startup,
shutdown, and the lock row are owned by
[../runtime/daemon-process.md](../runtime/daemon-process.md). Upgrades are
image rebuilds; the store and workspace survive in LKJAGENT_DATA_DIR.
Compose declares `restart: unless-stopped`, `init: true`, bounded json-file
logs, a 45-second stop grace window, and a healthcheck that verifies the
lkjagent process plus writable /data/workspace ownership.

## Network

Egress is allowed by default: the agent may curl, clone, and install. The
endpoint is reached by service name on the compose network. Owners who want
an airgapped agent restrict egress in compose without changing the harness.
Compose wiring is owned by
[../../operations/compose.md](../../operations/compose.md).

## Status

implemented.
