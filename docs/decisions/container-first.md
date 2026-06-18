# Container First

## Purpose

Fix where the harness runs and what the safety model is.

## Decision

The whole harness runs inside a container. The host runs only docker compose.
There is no host-mode installation, no permission prompt, and no approval
flow: the agent operates in YOLO mode because the container boundary is the
safety mechanism. The sandbox contract is
[../architecture/sandbox/README.md](../architecture/sandbox/README.md).

## Consequences

- The toolset needs no permission system, which keeps the loop simple and the
  context free of approval chatter.
- The blast radius of any action is the container filesystem, the mounted
  workspace, and the data directory; nothing else exists from the agent's
  view.
- Compose owns the wiring between the harness container and the endpoint;
  see [../operations/compose.md](../operations/compose.md).
- Upgrades are image rebuilds; the store and workspace survive in the data
  directory.

## Rejected Directions

- Harness on host with per-tool sandboxing: every tool becomes a security
  decision, and one mistake exposes the host; the container moves that
  decision to one well-understood boundary.
- Permission prompts: they assume an attending human, which contradicts a
  continuously running, queue-fed design.
