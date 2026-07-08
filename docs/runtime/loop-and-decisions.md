# Loop And Decisions

## Purpose

Define one state-ledger runtime cycle and the persisted decision record.

## Loop Shape

```text
Durable rows -> RuntimeSnapshot
RuntimeSnapshot + RuntimeEvent -> RuntimeDecision
RuntimeDecision -> PromptFrame or EffectCommand
PromptFrame + RuntimeDecision -> ModelCall
ModelAction + RuntimeDecision -> ToolAdmission
ToolAdmission -> EffectCommand
EffectObservation -> RuntimeEvent
ModelFreeDecision -> RuntimeEvent
RuntimeEvent -> durable rows
```

## Decision Rule

The selector creates bounded candidates from active cells, state edges, context
summaries, records, artifacts, and stale evidence. Exactly one `RuntimeDecision`
is selected for a turn and persisted before prompt rendering, endpoint calls,
tool-call admission, tool execution, recovery, compaction, or completion. The
decision id and fingerprint are carried through prompt frames, provider
exchanges, admissions, observations, events, checks, status output, and proof
bundles.

## Decision Fields

A decision stores case id, selected state key, selected operation key, snapshot
fingerprint, state-vector fingerprint, context-frame fingerprint, tool-view
fingerprint, expected envelope, model budget, admissible tool-call schemas,
hidden tool reasons for diagnostics, effect command when model-free, completion
predicates, and recovery policy. Settlement resolves or suppresses the selected
state key recorded on the decision; operation strings are not the authority for
which cell settled.

## Authority Rule

There is no prompt-only policy and no dispatcher-only policy. Prompt rendering
and admission are projections of the same persisted decision. A model-free
operation such as `state.resolve` may settle the selected state key directly
through the runtime loop without creating or reading a bridge step. A narrow
model-free effect such as `workspace.write_text` is carried on the persisted
decision as an effect command and is path-checked by the workspace effect edge.

## Acceptance Checks

- `lkjagent-core` selector tests prove operation priority, reused decisions, and
  prepared context-frame fingerprints.
- `lkjagent-app` prompt-frame and exchange tests prove the decision, prompt
  frame, exchange row, and replay body refs agree on fingerprints.
- Status output exposes the latest decision operation, status, context
  fingerprint, and tool-view fingerprint.

## Failure This Prevents

The model cannot be shown a tool that the harness refuses for the turn, and a
crash cannot rebuild different prompt authority for the same turn.
