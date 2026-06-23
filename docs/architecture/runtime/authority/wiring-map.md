# Wiring Map

## Purpose

Map every runtime path that can admit tools, recover, compact, maintain, or close
a case to the single authority reducer and admission gate.

## Rule

A path may collect facts, but it may not decide mission, permission, or closure.
Each path builds or refreshes `RuntimeSnapshot`, emits one `RuntimeEvent`, and
uses the resulting `RuntimeDecision` for prompt rendering and dispatch.

## Turn Path

```text
store facts + graph guidance + context facts
-> RuntimeSnapshot
-> RuntimeEvent
-> reducer
-> RuntimeDecision
-> ContextFrame
-> PromptFrame
-> model action intent
-> admission check from the same RuntimeDecision
-> effect or structured refusal
-> observation event
```

The dispatcher receives either the decision or an immutable admission view made
from it. It does not recompute policy from stale graph state.

## Close Paths

| Path | Required Authority Step |
| --- | --- |
| `agent.done` | `completion_requested` event, then central completion gate |
| graph complete node | graph suggestion becomes event input only |
| recovery handoff | completion gate or blocked partial handoff gate |
| turn budget exhausted | `turn_budget_exhausted` event, then blocked handoff gate |
| console close | current snapshot plus close event, then same gate |
| daemon shutdown handoff | current snapshot plus handoff event, then same gate |
| maintenance close | maintenance outcome gate, never owner-task completion |

No close path may treat planning, graph notes, file existence, manifest shape, or
scaffold topology as content readiness.

## Recovery Paths

Fault handlers classify parse, schema, admission, repeat, payload, audit,
verification, compaction, endpoint, and budget faults before the reducer runs.
The reducer selects the recovery mission, admitted escape tools, retry budget,
forced next action, fallback action, and exact schema example. Recovery output
must preserve at least one productive inspection, audit, repair, batch, smaller
scope, or blocked handoff route.

## Compaction Paths

Context pressure emits a runtime event. Hard compaction selects the compaction
mission before owner intake, recovery, maintenance, normal execution, or idle.
The persisted snapshot is written by runtime effects after the decision. The
model is not asked to run `memory.save` to preserve deterministic state.

## Maintenance Paths

Maintenance eligibility is a reducer decision. Queue rows, active owner cases,
recovery faults, verification repair, artifact repair, and hard compaction all
block maintenance. A no-op maintenance outcome sets cooldown without writing a
low-value memory row.

## Prompt Paths

Prompt rendering consumes `RuntimeDecision` plus `ContextFrame`. Allowed tools,
blocked tools, exact examples, completion blockers, missing evidence, and the
next action in the prompt must match the decision used by dispatch. Runtime
prompt cards include the persisted authority decision id and fingerprint that
also back the immutable dispatch admission view.

## Status

partially implemented. Daemon owner-task prompt cards and dispatch admission
views share the persisted authority decision id and fingerprint. Broader close,
recovery, compaction, maintenance, and console paths still need proof.
