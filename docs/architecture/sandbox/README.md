# Sandbox

## Purpose

This directory specifies the execution sandbox: one container that is the
safety model. Inside the boundary the agent runs YOLO, with no permission
prompts; outside the boundary nothing exists from the agent's view.
Decision:
[../../decisions/container-first.md](../../decisions/container-first.md).

## Table of Contents

- [container.md](container.md): the image, the user, the mounts, environment, and network.
- [workspace.md](workspace.md): /workspace semantics, the workspace brief, and the /data layout.
- [safety.md](safety.md): the YOLO posture, the blast radius, and owner responsibilities.
