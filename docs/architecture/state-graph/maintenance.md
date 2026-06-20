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

## Status

implemented.
