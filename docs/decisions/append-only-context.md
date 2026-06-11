# Append-Only Context

## Purpose

Fix how the context window is built, kept clean, and kept cheap.

## Decision

The context is a stable system prefix plus a strictly appended task log.
Between compactions nothing already sent is ever edited, reordered, or
removed, so the endpoint prefix cache hits on every turn. Compaction is an
explicit event that distills the log into memory, rebuilds the prefix, and
restarts the log. The layout is owned by
[../architecture/context/layout.md](../architecture/context/layout.md).

Every byte that enters the window has a budget and an owner per
[../architecture/context/budgets.md](../architecture/context/budgets.md) and
[../architecture/context/hygiene.md](../architecture/context/hygiene.md).

## Consequences

- Turn latency is dominated by generation, not prompt re-evaluation; a 32k
  window on local hardware stays interactive.
- Skill bodies and observations are immutable once appended; corrections
  append, never rewrite.
- Compaction is visible in transcripts as a first-class event, so context
  state is always reconstructible from the store.
- The harness, not the model, enforces budgets; the model never sees an
  overflow surprise.

## Rejected Directions

- Re-assembling budgeted regions every turn: cleaner windows per turn, but
  every reassembly invalidates the prefix cache and burns seconds of prompt
  processing on every action.
- Sliding-window truncation without distillation: silently forgets work; a
  prior project lost task state exactly this way.
- Letting the model manage its own window: spends scarce intelligence on
  bookkeeping the harness can do deterministically.
