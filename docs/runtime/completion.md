# Completion

## Purpose

Define obligations, native checks, final messages, and matter closure.

## Obligations

Owner intent compiles into required predicates. The initial edit predicates cover
target identity, current source revision, intended exact change, collateral path
set, effect settlement, and truthful response.

Obligation state is a reducer projection. A model message, empty work list,
elapsed time, or readiness phrase cannot satisfy it.

## Checks

Every committed create/edit event schedules native checks automatically. A check
stores matter, obligation, decision, parameters, current document revision,
measured result, evidence fingerprint, and causal sequence.

Initial file checks prove regular UTF-8 type, exact intended SHA-256, expected
old/new occurrence counts, admitted diff, preserved mode, allowed changed paths,
and no unsettled effect. A later revision invalidates dependent checks.

Shell checks do not exist until a state-limited journaled command effect is
adopted.

## Final Message

Only a respond decision accepts `<final>`. Mandatory admission rejects
future-tense readiness and unchecked path, effect, command, or verification
claims. The owner-visible output always includes a harness-rendered factual
receipt. If model wording remains invalid, the receipt alone is persisted.

## Close

Closure requires all required current checks passed, no pending/failed blocking
operation, no unsettled effect, and a final message. One transaction commits the
message, completion event, projection updates, and matter lifecycle.

The model never selects or computes closure.
