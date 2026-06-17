# Skill: Context Engine

## Purpose

Change window layout, budgets, compaction, or caching without breaking the
two invariants: byte-monotonic requests between compactions, and every
frame budgeted and owned.

## Trigger

Window layout, budgets, compaction, or caching rules are changing.

## Context

- [../../architecture/context/layout.md](../../architecture/context/layout.md): regions and message mapping.
- [../../architecture/context/budgets.md](../../architecture/context/budgets.md): the ledger every number lives in.
- [../../architecture/context/compaction.md](../../architecture/context/compaction.md): the rebuild procedure.
- [../../architecture/context/caching.md](../../architecture/context/caching.md):
  byte stability and lawful invalidations.
- [../../architecture/context/hygiene.md](../../architecture/context/hygiene.md): the admission allowlist.

## Procedure

1. Write the change into the owning context doc first. A new frame kind
   needs a hygiene allowlist row; a new number needs a ledger row with an
   owner and an overflow rule; a new invalidation needs a caching table row.
2. Re-run the budget arithmetic: prefix total, log floor (16,384 minimum),
   trigger, and post-compaction target must still cohere; the startup
   assertion in budgets.md states the floor.
3. Implement in lkjagent-context as pure functions: admission, accounting,
   trigger, and rebuild decisions all take state and return decisions.
4. Extend the byte-monotonicity test: serialize consecutive requests across
   the changed behavior and assert the earlier request is a strict prefix
   of the later, except across a compaction.
5. Extend compaction tests: a synthetic over-budget state compacts to under
   the target, the task summary lands at the log head, and the transcript
   event records before and after counts.
6. If notices changed shape, update hygiene.md's kind table and the
   transcript kind expectations in
   [../../architecture/memory/transcripts.md](../../architecture/memory/transcripts.md).

## Checks

- `cargo test -p lkjagent-context` passes, including the prefix-stability
  property and the compaction-reaches-target test.
- Ledger arithmetic in budgets.md sums correctly (the doc table and the
  code constants are asserted equal by a test).
- `cargo run -p lkjagent-xtask -- quiet verify` prints ok verify.

## Must Not

- Do not introduce any per-turn varying bytes into frames: no timestamps,
  no counters, no random ids in window content.
- Do not add a frame kind without a budget owner and an overflow rule.
- Do not let compaction lose state silently; failure to reach target is a
  loud error by contract.
- Do not tune numbers in code without moving the ledger table in the same
  commit.
