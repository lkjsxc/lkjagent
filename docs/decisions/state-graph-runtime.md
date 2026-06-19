# State Graph Runtime

## Purpose

Fix the central model for agent behavior.

## Decision

lkjagent runs on a typed state-transition graph. The graph classifies owner
work, creates durable cases, selects context packages, gates transitions,
records evidence, drives compaction and recovery, and refuses completion when
required evidence is missing. The graph contract is
[../architecture/state-graph/README.md](../architecture/state-graph/README.md).

## Consequences

- The first owner message starts classification and planning; it is never
  enough to begin free execution by itself.
- The prefix renders a compact graph slice instead of a procedure index.
- Graph state, events, and evidence are persisted in SQLite so restart and
  compaction resume from structured state.
- Idle maintenance improves graph patterns, context packages, policy, memory,
  and tests.
- The runtime remains one daemon, one loop, no MCP, and no sub-agents.

## Rejected Directions

- Flat procedure files as the capability model: they rely on model initiative
  and do not encode legal state transitions or evidence gates.
- Prompt-only planning reminders: weak local models skip them under pressure.
- Mutable runtime prompt sprawl: it obscures provenance and cannot be tested
  as deterministic graph data.
