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

A committed create/edit event schedules native checks automatically. The reducer
requires a settled successful observation and no model turn. Required
`workspace-byte`, `workspace-content`, and
`workspace-collateral` obligations carry canonical JSON parameters for the path
and SHA-256, old/new counts, or allowed path set. The harness rereads through the
opened workspace capability and stores matter, obligation, original decision,
parameters, measured bytes and mode, source revision, evidence fingerprint, and
a fresh causal event.

Scheduling is one immediate transaction. Deterministic identities make a repeat
for the same journal and source revision a no-op. A new observed revision makes
the prior check non-current before inserting its replacement. Initial file
checks prove UTF-8 content counts, exact intended SHA-256 and bytes, preserved
mode, and admitted collateral paths. A failed replacement remains current and
keeps its obligation open.

Every read states `complete=false` when an edit may follow or `complete=true` for
a no-change report objective. A final envelope
sent one phase early is rejected, but can trigger the same checked transition
after a successful current read. The store accepts either path only when the
opened file revision equals the current managed revision from a closed matter
whose required checks remain current and passed. It copies those facts into new
matter-bound obligations and checks tied to the settling read decision and
event; it never treats an unchecked read or premature wording as completion.

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
