# Context Engine

## Purpose

Implement lkjagent-context: the token ledger, frame admission, the
append-only message list, the compaction trigger and rebuild decisions, all
as pure functions.

## Status

done with Implement pure context engine decisions

## Depends On

[protocol-parser.md](protocol-parser.md) (frame types).

## Files To Read

- [../../architecture/context/layout.md](../../architecture/context/layout.md)
- [../../architecture/context/budgets.md](../../architecture/context/budgets.md)
- [../../architecture/context/compaction.md](../../architecture/context/compaction.md)
- [../../architecture/context/caching.md](../../architecture/context/caching.md)
- [../../architecture/context/hygiene.md](../../architecture/context/hygiene.md)
- [../../agent/skills/context-engine.md](../../agent/skills/context-engine.md)

## Files To Touch

- crates/lkjagent-context/src/: model.rs (regions, frames, ledger),
  budget.rs (the ledger table as code, asserted against the doc),
  admission.rs (allowlist and caps, truncation decisions), assemble.rs
  (deterministic serialization to the message list), compaction.rs
  (trigger and rebuild plan).
- crates/lkjagent-context/tests/: budget arithmetic, admission, prefix
  stability, compaction-reaches-target tables.

## Focused Gate

```sh
cargo test -p lkjagent-context
cargo clippy -p lkjagent-context -- -D warnings
```

## Acceptance

- The byte-monotonicity property passes: consecutive assembled requests are
  strict prefixes except across compaction.
- A synthetic over-budget state produces a rebuild plan landing at or under
  the post-compaction target, with the task summary at the log head.
- Every budget row in budgets.md has a code constant and a test asserting
  doc and code agree.
- Truncation decisions always carry the retrieval-path notice content.
- Blocker row 4 done; context area status moves in the ledger.

## Must Not

- Do not perform IO or token counting against a live endpoint; counts
  arrive as inputs.
- Do not emit any frame content with per-turn variance.
- Do not let admission silently drop anything; every refusal is a decision
  value the runtime renders as a notice.
