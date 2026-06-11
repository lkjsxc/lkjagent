# No Sub-Agents

## Purpose

Fix the boundary against sub-agents, worker forks, and delegation trees.

## Decision

The harness never spawns a second agent loop. No sub-agents, no worker
forks, no parallel explorations, no delegation. The single loop in
[single-loop.md](single-loop.md) is the only consumer of model output.

## Consequences

- The 16 GB / 32k budget serves one context well instead of several badly;
  a sub-agent would halve the cache and double the prompt processing.
- No result-merging logic, no orphaned workers, no cross-loop state races.
- Long tasks decompose in time, not in workers: the loop works sequentially
  and compaction carries state across phases per
  [append-only-context.md](append-only-context.md).
- Work that truly needs parallelism (builds, searches, test runs) parallelizes
  inside shell commands, where the operating system already does it well.

## Rejected Directions

- Bounded worker forks with their own windows: every fork restarts a cold
  cache on a machine whose whole design centers on keeping one cache hot.
- Out-of-process helper agents: reintroduces orchestration, scheduling, and
  merge complexity that the smallness principle in
  [../vision/principles.md](../vision/principles.md) exists to forbid.
