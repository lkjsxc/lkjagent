# Structural Truth Sweep

## Purpose

Reconcile documentation status before the CLI, token, resolver, atom, and
manuscript implementation slices depend on it.

## Status

done: documentation reconciled; `check-docs` and `check-lines` passed in this
session

## Depends On

None.

## Files To Read

1. [../../../README.md](../../../README.md)
2. [../../../AGENTS.md](../../../AGENTS.md)
3. [../../current-state.md](../../current-state.md)
4. [../current-blockers.md](../current-blockers.md)
5. [../current-work/story-manuscript-generation-gap.md](../current-work/story-manuscript-generation-gap.md)
6. [Protocol action format](../../architecture/protocol/action-format.md)
7. [Batch write](../../architecture/protocol/batch-write.md)
8. [Output budget](../../architecture/llm/output-budget.md)
9. [Product observability](../../product/observability.md)
10. `tmp/lkjagent-redesign-handoff/03-implementation-tasks/task-00-truth-sweep.md`

## Files To Touch

- `docs/current-state.md`
- `docs/execution/current-blockers.md`
- `docs/execution/tasks/README.md`
- `docs/execution/tasks/structural-truth-sweep.md`
- new task docs for the active redesign queue
- status sections in protocol, LLM, and observability docs
- `docs/_meta/catalog/execution.toml`

## Focused Gate

```sh
cargo run -p lkjagent-xtask -- check-docs
cargo run -p lkjagent-xtask -- check-lines
```

## Acceptance

- Long daemon manuscript completion is not claimed without daemon proof.
- Protocol and output-budget status sections match current implemented behavior.
- Token accounting docs separate latest usage from open cumulative aggregates.
- Current blockers name the new dependency-ordered redesign queue.
- New task docs are linked from the task README and catalog.

## Must Not

- Do not change runtime behavior.
- Do not mark later redesign tasks done from documentation edits.
- Do not claim a gate passed unless it ran in the current session.
