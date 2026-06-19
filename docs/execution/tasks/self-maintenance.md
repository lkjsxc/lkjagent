# Self-Maintenance

## Purpose

Implement explicit maintenance helpers: directive rotation with staleness
weighting, cycle budgets, normal YOLO authority during maintenance work,
and the distillation moments at task close and compaction.

## Status

done

## Depends On

[agent-loop.md](agent-loop.md), [queue-cli.md](queue-cli.md) (a running
daemon to extend).

## Files To Read

- [../../architecture/runtime/self-maintenance.md](../../architecture/runtime/self-maintenance.md)
- [../../architecture/memory/distillation.md](../../architecture/memory/distillation.md)
- [../../architecture/skills/lifecycle.md](../../architecture/skills/lifecycle.md)
- [../../architecture/tools/registry.md](../../architecture/tools/registry.md)
  (the shared tool authority)

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

- The runtime can open an explicit maintenance cycle with the stalest
  directive, bounded by the cycle budget, asserted in scripted tests.
- A queue arrival preempts an explicit maintenance cycle at the next
  boundary, never mid-turn.
- Workspace fs and shell actions during maintenance succeed under the same
  authority as task work; there is no maintenance-specific block.
- Maintenance can mutate queue rows through queue tools; each mutation has
  a queue_mutation transcript event and respects cycle budgets.
- External write behavior is tested with local fixtures such as a bare git
  remote, not live production services.
- Task close saves a task-summary row; compaction allows up to four
  `memory.save` turns and falls back to a harness-written task-summary when
  an open task still needs one.
- Directive stamps update in the state table; the rotation is
  deterministic given the stamps.
- Blocker row 11 done; self-maintenance status moves in the ledger.

## Must Not

- Do not add timers or schedules; automatic idle maintenance is driven only
  by the resident poll loop.
- Do not let maintenance turns bypass any budget or hygiene rule; there is
  no budget exemption.
- Do not fabricate maintenance outcomes; an empty cycle ends honestly.
