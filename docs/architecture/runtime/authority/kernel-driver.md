# Kernel Driver

## Purpose

Define the effectful runtime driver that sequences durable reads, kernel
reduction, prompt rendering, provider exchange, admission, dispatch, and
observation without becoming a second policy engine.

## Contract

The driver owns order. The pure kernel owns decisions. Every turn starts from
store facts, emits one event, persists one decision, and only then performs a
prompt, provider, runtime effect, dispatch, maintenance, compaction, status, or
close operation.

```text
DurableReadModel -> RuntimeSnapshot
RuntimeSnapshot + RuntimeEvent -> RuntimeDecision
RuntimeDecision -> PromptFrame or RuntimeEffectCommand
RuntimeDecision + ModelAction -> ToolAdmission
ToolAdmission -> RuntimeEffectCommand
RuntimeEffectCommand -> EffectObservation
EffectObservation -> RuntimeEvent
RuntimeEvent -> next RuntimeDecision
```

## Sequence

1. Read durable facts for queue, case, graph, evidence, artifact, fault,
   observation, context, maintenance, prompt, authority, and provider state.
2. Build `RuntimeSnapshot` without synthetic authority defaults.
3. Record the triggering `RuntimeEvent`.
4. Reduce snapshot plus event into `RuntimeDecision`.
5. Persist the decision with authority and staleness fingerprints.
6. Persist a prompt frame when the decision needs model-authored content.
7. Persist a runtime effect command when the decision is deterministic.
8. Execute only the persisted prompt or runtime effect.
9. Classify provider output before parsing action text.
10. Convert parse result, parse fault, or provider anomaly into the next event
    source.
11. Build an immutable admission view from the persisted decision.
12. Persist accepted or refused admission before any tool adapter runs.
13. Dispatch only accepted effect commands.
14. Persist observation rows and emit the observation event.

## Source Adapters

- queue adapter emits owner queue events.
- prompt adapter emits frame rendered or frame skipped events.
- provider adapter emits response, endpoint error, or anomaly events.
- parser adapter emits action parsed, parse fault, or envelope normalization
  events.
- dispatcher adapter emits admission and tool observation events.
- graph, artifact, verification, compaction, maintenance, and completion
  adapters emit facts only.

## Consumers

Prompt rendering, model-log export, status, admission, dispatch, compaction,
maintenance, recovery, and close paths all consume the same decision id.
Graph policy is snapshot guidance only. The old mode tree may format fields or
fill non-authority snapshot facts, but it must not choose missions, tool
surfaces, recovery routes, or close outcomes.

## Invariants

- A provider call has stored snapshot, event, decision, and prompt frame rows
  before the endpoint request is written.
- A runtime effect has stored snapshot, event, decision, and effect command rows
  before it runs.
- Model-call decisions always admit at least one tool.
- Empty admitted tool sets are limited to deterministic effects, owner wait,
  case close, and closed idle.
- A refused action never reaches a tool adapter.

## Status

implemented for focused daemon tests, pending final gates. The runtime driver
persists snapshot, event, decision, prompt frame or runtime effect, admission,
effect, and observation rows before the next effectful step. The daemon uses
this driver for endpoint turns and pending dispatch. Final verification must
prove the same path across the full workspace and Docker gate.
