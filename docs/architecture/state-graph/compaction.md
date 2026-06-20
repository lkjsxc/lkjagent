# Graph Compaction

## Purpose

Specify compaction as a typed state transition with preservation rules, not
only a token-pressure event.

## Preservation

A `CompactionPlan` preserves:

- active case id, objective, non-goals, active node, phase, and plan steps.
- constraints, risks, success criteria, evidence gathered, missing evidence,
  and touched paths.
- selected packages, package compression, legal next transitions, recent
  failures, recovery strategy, and completion readiness.

The restart notice is rendered from this structured state. A task summary row
may exist for memory retrieval, but it is not the source of truth.

## Pressure Routing

The context engine reports green, yellow, orange, red, or black-invalid
pressure. The graph policy decides whether to keep going, narrow package
selection, schedule compaction, compact immediately, or pause.

Phase-boundary compaction is allowed when the graph policy says the next phase
needs a clean window. Forced compaction is a runtime transaction and does not
ask the model to run `memory.save`.

Yellow narrows optional package text. Orange schedules a checkpoint at the
next safe phase boundary. Red compacts before the next endpoint call or owner
handoff. Black-invalid pauses only when structured state cannot produce a
valid context window.

Compaction policy cannot contradict tool policy. If model-authored
distillation is desired, the active node must admit memory tools. Otherwise
the runtime snapshots state and rebuilds the prefix without model action.

## Status

implemented.
