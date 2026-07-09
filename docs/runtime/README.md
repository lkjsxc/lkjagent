# Runtime

## Purpose

Define the durable decision loop that projects state into prompts, effects,
recovery, and completion.

## Table of Contents

- [loop-and-decisions.md](loop-and-decisions.md): snapshot hydration,
  selector candidates, `RuntimeDecision` fields, and prompt or effect selection.
- [recovery-and-completion.md](recovery-and-completion.md): crash recovery,
  evidence-gated closure, observability, and proof requirements.
- [artifact-units.md](artifact-units.md): small generation units,
  deterministic assembly, artifact manifests, and artifact-fingerprint checks.

## Runtime Contract

Each cycle projects fresh durable state before choosing work. The selected
`RuntimeDecision` stores a derived harness state such as `intake`, `act`,
`recover`, or `idle`; prompt cards and tool exposure are projections of that same
row. Earlier blocked, active, failed, pending, or unsuperseded skipped cells are
preflight blockers for later model response work. Recovery or supersession
evidence must settle the blocker before the selector can project a happy response
or close candidate.

## Failure This Prevents

Prompt rendering, tool admission, effect dispatch, recovery, resume, and
completion all use the same persisted authority row for a turn.
