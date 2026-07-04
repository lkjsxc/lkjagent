# Runtime

## Purpose

Define the durable decision loop that projects state into prompts, effects,
recovery, and completion.

## Table of Contents

- [loop-and-decisions.md](loop-and-decisions.md): snapshot hydration,
  `RuntimeDecision` fields, and prompt or effect selection.
- [recovery-and-completion.md](recovery-and-completion.md): crash recovery,
  evidence-gated closure, observability, and proof requirements.

## Failure This Prevents

Prompt rendering, tool admission, effect dispatch, and resume all use the same
persisted authority row for a turn.
