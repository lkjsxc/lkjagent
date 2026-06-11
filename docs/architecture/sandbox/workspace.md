# Workspace

## Purpose

The two directory trees the agent can see: /workspace, the owner's project
under work, and /data, the harness's own state. Everything else in the
container is image content that an image rebuild resets.

## The /workspace Mount

/workspace is a bind mount of the project the owner chose when starting the
container. The agent may assume:

- it is a git checkout the agent fully owns,
- every file in it may be read, edited, created, or deleted,
- branches, commits, and tool runs inside it are the agent's to make,
- nothing outside /workspace and /data exists from the agent's view.

Choosing what to mount is the owner's safety decision per
[safety.md](safety.md).

## The Workspace Brief

/workspace/AGENTS.md is the workspace brief: the project's own standing
instructions. It is loaded verbatim into the prefix under a 1,024-token
cap; on overflow the head is kept and a truncation notice marks the cut,
per [../context/budgets.md](../context/budgets.md).

## The /data Volume

| Path | Holds |
| --- | --- |
| /data/lkjagent.sqlite3 | the store, per [../memory/store.md](../memory/store.md) |
| /data/skills | the skill library: markdown skill files indexed by the store |
| /data/lkjagent.toml | the config file read at daemon startup |

/data is a named volume, so the store and the skill library survive image
rebuilds and container replacement. Snapshotting the volume is the owner's
rollback mechanism per [safety.md](safety.md).

## Status

design-only.
