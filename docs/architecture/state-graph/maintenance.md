# Graph Maintenance

## Purpose

Specify idle self-maintenance as graph evolution, policy cleanup, and memory
quality work.

## Directives

Idle maintenance rotates through:

- distill: save reusable lessons, facts, task summaries, and incidents.
- refine-graph-policy: record graph policy and context package improvement
  candidates from observed failures and gaps.
- prune-memory: merge, correct, or delete stale durable memory.
- audit-self: record mismatches between docs, code, graph state, and tests.

Maintenance follows the same graph state machinery as owner tasks but is
limited to memory and graph-note style actions by the maintenance gate. It
records policy candidates; it does not edit source while idle.

Maintenance runs only after active-mode selection proves no owner work is
pending, active, or recoverable. No-op maintenance sets cooldown and writes no
memory row. Pruning and merge claims must name the row IDs changed by the
store operation.

## Status

partially implemented. Exact duplicate pruning exists; semantic merge and
rewrite pruning remain open.
