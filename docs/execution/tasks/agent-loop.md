# Agent Loop

## Purpose

Implement lkjagent-runtime: the pure step function composing protocol,
graph, context, store, llm, and tools into the turn lifecycle, plus the
daemon shell around it: startup, lock, signals, intake, recovery escalation.

## Status

done

## Depends On

All of rows 3 through 8; this is the integration point and must not start
while any of them is open.

## Files To Read

- [../../architecture/runtime/agent-loop.md](../../architecture/runtime/agent-loop.md)
- [../../architecture/runtime/daemon-process.md](../../architecture/runtime/daemon-process.md)
- [../../architecture/runtime/queue-intake.md](../../architecture/runtime/queue-intake.md)
- [../../architecture/protocol/recovery.md](../../architecture/protocol/recovery.md)
- [../../architecture/protocol/system-prompt.md](../../architecture/protocol/system-prompt.md)

## Files To Touch

- crates/lkjagent-runtime/src/: step.rs (the pure transition), task.rs
  (task and waiting states, budgets), intake.rs, recovery.rs (fault
  counters, escalation), prompt.rs (prefix assembly from the static
  template plus generated sections), daemon.rs (startup order, lock,
  signal handling, the turn driver), error.rs.
- crates/lkjagent-runtime/tests/: scripted-completion tables driving step
  through whole task lifecycles without IO; daemon tests for lock and
  signal behavior.

## Focused Gate

```sh
cargo test -p lkjagent-runtime
cargo clippy -p lkjagent-runtime -- -D warnings
cargo run -p lkjagent-xtask -- quiet verify
```

## Acceptance

- A scripted task runs end to end in tests: owner frame, turns, tool
  effects, agent.done, distillation prompt, all asserted as values.
- Recovery escalation records stronger notices after consecutive
  parse-class faults while keeping the task open, with the documented
  events.
- Compaction integrates: a scripted over-budget session distills, rebuilds,
  and resumes, with the transcript event recorded.
- The prefix assembled by prompt.rs is byte-identical across two builds
  from the same state, and within every section budget.
- Startup resumes an open task from its summary without replaying raw
  events.
- Blocker row 9 done; runtime and daemon area statuses move in the ledger.

## Must Not

- Do not put IO in step.rs; effects are values interpreted by daemon.rs.
- Do not start this task with any dependency row open.
- Do not invent turn behavior absent from the contracts; gaps go back to
  docs first.
