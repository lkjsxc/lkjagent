# Transition Kernel

## Purpose

Define the deterministic turn kernel that makes runtime authority durable before
any prompt rendering, provider call, tool dispatch, or completion closure.

## Contract

One kernel owns the turn sequence:

```text
snapshot -> event -> decision -> prompt frame or runtime effect
provider exchange -> parse -> admission -> effect -> observation -> next event
```

The model proposes intent inside the provider exchange. The runtime decision is
the source of active mode, admitted tools, recovery route, completion gate,
runtime-owned deterministic effects, and prompt text. Graph policy is input to
the snapshot, not a fallback authority.

## Inputs

The snapshot adapter collects these facts before the reducer runs:

- queue head and owner message count.
- actual case id, graph node id, graph phase, objective, constraints,
  assumptions, and risks; synthetic case ids such as `case:unknown` are invalid.
- graph policy, ranked tracks, missing evidence, existing evidence, and legal
  transitions as inputs only, never as fallback dispatch authority.
- artifact root, artifact head, weak paths, weak-path cursor, write batch
  cursor, audit id, audit status, and drift state.
- latest fault, retry counters, failed action fingerprint, and recovery route.
- latest observation and latest successful observation.
- compaction pressure, compaction head, and maintenance state.
- latest decision id, prompt frame fingerprint, and staleness fingerprint.
- queue, case, graph, active-mode, artifact, fault, evidence, compaction,
  maintenance, and prompt-frame facts that can invalidate a cached action.

## Decision Data

The pure reducer emits one persisted decision for one event. The decision names:

- active mission and active mode.
- admitted tools and blocked tools.
- forced next action or exact valid example.
- missing and existing evidence.
- recovery, compaction, maintenance, blocked-handoff, and completion data.
- authority fingerprint and staleness fingerprint.

Prompt frames render only from this persisted decision. Dispatch receives an
immutable admission view derived from the same decision id. Deterministic
runtime effects such as compaction, closed-idle wait, maintenance defer,
blocked-handoff recording, status refresh, or zero-content inspection tools are
also emitted from the persisted decision and do not require a model-authored
`<act>`.

## Admission Data

Admission compares the parsed action with the decision-derived view. It records:

- requested tool and action fingerprint.
- accepted or refused status.
- refusal reason and exact valid example.
- schema, payload, repeat-action, and staleness findings.
- dispatch plan when accepted.

A refused action never reaches a tool adapter. A stale action records a refusal
and triggers a fresh snapshot event before the model sees another frame.

## Persistence Order

The runtime writes records in this order:

1. runtime snapshot.
2. runtime event.
3. runtime decision.
4. prompt frame reference and fingerprint.
5. provider exchange request record.
6. provider exchange response or error record.
7. parsed action or parse fault.
8. admission record.
9. effect and observation records when admitted.
10. next runtime event.

## Invariants

- One event emits one decision.
- One prompt frame cites one decision id.
- A tool-requiring next action always has a non-empty admitted tool set.
- Empty tool sets pair only with runtime compaction, admitted completion,
  external owner wait, or idle.
- Maintenance is never active during owner work, recovery, verification, or compaction.
- Completion uses the central completion reducer on every close path.
- Recovery keeps the read, audit, repair, and batch tools needed to escape.

## Verification

Focused tests cover the pure reducer, prompt rendering from a decision id,
admission from an immutable view, stale-action refusal by full fingerprint,
maintenance preemption, completion refusal, runtime-owned compaction or
inspection effects, and recovery escape-tool visibility.

## Status

partially implemented. Snapshot, event, decision, transition, effect,
admission, prompt-frame, and observation ledgers exist. Runtime turn authority
writes normalized rows and prompt cards cite the decision id. Store tests prove
latest snapshot and decision reopen lookup and admission-to-decision integrity.
Snapshot adapter tests prove staleness changes for queue, artifact cursor, and
compaction pressure, ignore maintenance due state during owner work, and reject
synthetic active case ids. Full kernel wiring for every dispatch, provider
exchange, recovery, compaction, maintenance, and close path remains open.
