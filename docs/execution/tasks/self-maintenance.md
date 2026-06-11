# Self-Maintenance

## Purpose

Implement the idle loop: directive rotation with staleness weighting,
cycle budgets, maintenance-mode tool restrictions, and the distillation
moments at task close and compaction.

## Status

open

## Depends On

[agent-loop.md](agent-loop.md), [queue-cli.md](queue-cli.md) (a running
daemon to extend).

## Files To Read

- [../../architecture/runtime/self-maintenance.md](../../architecture/runtime/self-maintenance.md)
- [../../architecture/memory/distillation.md](../../architecture/memory/distillation.md)
- [../../architecture/skills/lifecycle.md](../../architecture/skills/lifecycle.md)
- [../../architecture/tools/registry.md](../../architecture/tools/registry.md)
  (the maintenance restriction)

## Files To Touch

- crates/lkjagent-runtime/src/: maintenance.rs (directive choice from
  state stamps, cycle budget), extensions to step.rs for the maintenance
  notice and the distillation prompts at task close and compaction.
- crates/lkjagent-store/src/state.rs: per-directive stamps if not already
  present.
- crates/lkjagent-runtime/tests/: scripted idle cycles per directive;
  preemption by a queue arrival; the empty-cycle early agent.done path.

## Focused Gate

```sh
cargo test -p lkjagent-runtime
cargo run -p lkjagent-xtask -- quiet verify
```

## Acceptance

- An idle daemon opens a maintenance cycle with the stalest directive,
  bounded by the cycle budget, asserted in scripted tests.
- A queue arrival preempts at the next boundary, never mid-turn.
- Workspace-targeting fs and shell actions during maintenance return the
  documented tool error; memory and skill writes succeed.
- Task-close distillation injects the prompt and allows two turns;
  compaction distillation allows four and requires the task summary.
- Directive stamps update in the state table; the rotation is
  deterministic given the stamps.
- Blocker row 11 done; self-maintenance status moves in the ledger.

## Must Not

- Do not add timers, heartbeats, or schedules; idleness is the only
  trigger.
- Do not let maintenance turns bypass any budget or hygiene rule; there is
  no privileged mode.
- Do not fabricate maintenance outcomes; an empty cycle ends honestly.
