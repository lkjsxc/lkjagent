# Honest State

## Purpose

The truth rule of the project, in one place. Everything else links here.
It binds the coding agents building lkjagent and the running agent alike,
and it is the first principle in [../vision/principles.md](../vision/principles.md).

## The Rule

Nothing in this project may present a state that did not actually happen.

## For the Builders

- No mock implementations, no stubbed returns, no placeholder bodies. A
  function that cannot be implemented yet does not get merged; the task
  stays open instead.
- No fabricated test fixtures that assert behavior the system does not
  have; fixtures are recorded from real runs or constructed to the written
  contract, and say which.
- No claiming an unrun gate. Tested trailers and handoff reports quote
  actual output, per [handoff.md](handoff.md).
- No docs describing unbuilt behavior as existing: design-only status marks
  the line, and [../current-state.md](../current-state.md) is the ledger.
- Deleting something is honest; hiding it behind a flag that pretends is not.

## For the Running Agent

- agent.done summaries claim only what observations showed; the harness
  transcript is the audit trail.
- Observations are never synthesized: a tool that failed reports failure,
  per [../architecture/protocol/recovery.md](../architecture/protocol/recovery.md).
- Memory rows record what happened, not what should have happened; a
  distillation that smooths over a failure poisons every future retrieval.
- Graph policy changes name the observed evidence that justified them.
- Truncation, budget exhaustion, and compaction are announced in notices,
  never silent, per [../architecture/context/hygiene.md](../architecture/context/hygiene.md).

## The Discovery Corollary

Missing evidence never proves absence. An empty search result is a reason
to search differently, not a license to claim nonexistence; a cache miss is
a discovery trigger. State what was checked and what was found, and keep
the difference between "absent" and "not found by this method" explicit.

## Why This Is Rule One

An agent system compounds its own outputs: transcripts become memory,
memory becomes context, context becomes behavior. One fabricated success
poisons the chain at the root. Honesty here is not etiquette; it is the
load-bearing wall.
