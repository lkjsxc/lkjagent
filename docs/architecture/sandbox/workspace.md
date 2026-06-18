# Workspace

## Purpose

The data volume layout the agent can see: /data/workspace for the project
under work and sibling paths for harness state. Everything else in the
container is image content that an image rebuild resets.

## The /data/workspace Directory

/data/workspace is the project directory the daemon uses for shell and file
tools. It lives inside the data volume. The agent may assume:

- it is a git checkout the agent fully owns,
- every file in it may be read, edited, created, or deleted,
- branches, commits, and tool runs inside it are the agent's to make,
- nothing outside /data/workspace and /data exists from the agent's view.

Choosing to add extra mounts is the owner's safety decision per
[safety.md](safety.md).

## The Workspace Brief

/data/workspace/AGENTS.md is the workspace brief: the project's own standing
instructions. It is loaded verbatim into the prefix under a 1,024-token
cap; on overflow the head is kept and a truncation notice marks the cut,
per [../context/budgets.md](../context/budgets.md).

## The /data Volume

| Path | Holds |
| --- | --- |
| /data/lkjagent.sqlite3 | the store, per [../memory/store.md](../memory/store.md) |
| /data/workspace | the project directory the agent works in |
| /data/skills | the skill library: markdown skill files indexed by the store |
| /data/lkjagent.json | the config file read at daemon startup |

/data is a named volume, so the store, workspace, skill library, and config
survive image rebuilds and container replacement. Snapshotting the volume is
the owner's rollback mechanism per [safety.md](safety.md).

## Status

implemented.
