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
- git, curl, and ripgrep.

Nothing else is promised. The agent runs as a non-root user named agent.
There is no docker socket, and no host filesystem is visible beyond the two
mounts below.

## Mounts

| Mount | Kind | Holds |
| --- | --- | --- |
| /data | named volume | store file, skill library, config; layout in [workspace.md](workspace.md) |
| /workspace | bind mount | the owner's chosen project; semantics in [workspace.md](workspace.md) |

## Environment

| Variable | Meaning |
| --- | --- |
| endpoint URL | where the chat-completions endpoint is reached |
| model | the model name sent on every request |
| API key | optional; sent to the endpoint when set |

The API key, when needed, arrives by environment variable only: never baked
into the image and never written to the store, per
[../memory/store.md](../memory/store.md).

## Supervision

The container supervisor restarts the daemon on nonzero exit; startup,
shutdown, and the lock row are owned by
[../runtime/daemon-process.md](../runtime/daemon-process.md). Upgrades are
image rebuilds; the store and the workspace survive on their mounts.

## Network

Egress is allowed by default: the agent may curl, clone, and install. The
endpoint is reached by service name on the compose network. Owners who want
an airgapped agent restrict egress in compose without changing the harness.
Compose wiring is owned by
[../../operations/compose.md](../../operations/compose.md).

## Status

design-only.
