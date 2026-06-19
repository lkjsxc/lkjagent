# Graph Maintenance

## Purpose

Specify idle self-maintenance as graph evolution, policy cleanup, and memory
quality work.

## Directives

Idle maintenance rotates through:

- distill: save reusable lessons, facts, task summaries, and incidents.
- refine-skills: record source skill improvement candidates from observed
  failures and gaps.
- prune-memory: merge, correct, or delete stale durable memory.
- audit-self: record mismatches between docs, code, graph state, and tests.

Maintenance follows the same graph state machinery as owner tasks. It opens a
bounded case, records evidence, and closes only through the completion gate.

## Status

implemented.
