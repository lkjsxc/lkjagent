# Safety

## Purpose

The safety posture stated plainly: the container boundary is the entire
safety model, the agent runs without permission prompts inside it, and the
owner controls risk by controlling what crosses the boundary.

## YOLO Inside the Boundary

There are no permission prompts and no approval flows, by design:
[../../decisions/container-first.md](../../decisions/container-first.md).
Prompts assume an attending human, which contradicts a continuously
running, queue-fed agent. Inside the container the agent edits, executes,
installs, and reaches the network without asking. Task turns and explicit
maintenance paths have the same authority; maintenance has no extra sandbox
beyond this boundary.

## Blast Radius

The blast radius of any agent action is exactly:

- the container filesystem, reset by an image rebuild,
- /data/workspace, the project directory,
- /data, the mounted store, workspace, skill library, and config.

Nothing outside the mounts exists from the agent's view: no host
filesystem, no docker socket, no other process namespaces. Other compose
services exist only as network peers.

## Owner Responsibilities

- Choose mounts deliberately: mounting anything mounts it into the blast
  radius.
- Treat credentials handed to the agent as spendable: an API key in the
  environment, a token in the workspace, a remote the checkout can push to.
- Snapshot the host data directory for rollback: it contains the state worth
  keeping, per [workspace.md](workspace.md).
- Restrict egress in compose when the agent must be airgapped, per
  [container.md](container.md).

## What the Harness Still Refuses

The boundary does not excuse dishonesty. Regardless of sandbox posture, the
harness refuses fabricated results: claims of work not done, silent loss,
and invented observations, per
[../../agent/honest-state.md](../../agent/honest-state.md).

## Status

implemented.
