# Graph Compaction

## Purpose

Specify compaction as a typed state transition with preservation rules, not
only a token-pressure event.

## Preservation

A `CompactionPlan` preserves:

- active case id, objective, active node, phase, and plan steps.
- constraints, evidence gathered, missing evidence, and touched paths.
- recent failures, recovery strategy, selected packages, and completion
  readiness.

The restart notice is rendered from this structured state. A task summary row
may exist for memory retrieval, but it is not the source of truth.

## Pressure Routing

The context engine reports green, yellow, orange, red, or black-invalid
pressure. The graph policy decides whether to keep going, narrow package
selection, schedule compaction, compact immediately, or pause.

Phase-boundary compaction is allowed when the graph policy says the next phase
needs a clean window.

## Status

implemented.
