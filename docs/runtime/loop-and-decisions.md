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

A decision stores case id, selected state key, derived harness state, selected
operation key, snapshot fingerprint, state-vector fingerprint, context-frame
fingerprint, tool-view fingerprint, expected envelope, model budget, admissible
tool-call schemas, hidden tool reasons for diagnostics, effect command when
model-free, completion predicates, and recovery policy. Settlement resolves or
suppresses the selected state key recorded on the decision; operation strings are
not the authority for which cell settled.

## Harness States

Harness state is derived from the persisted decision. It is evidence for prompt
fragments and tests, not a second graph. The current states are `intake`,
`clarify`, `plan`, `act`, `observe`, `recover`, `record`, `maintain`, and
`idle`.

| State | Entry evidence | Exit evidence |
| --- | --- | --- |
| `intake` | owner-intake cell or operation | route row, transcript, inbox trace, record row, or clarify cell |
| `clarify` | waiting answer or message envelope | owner answer, question event, or supersession |
| `plan` | plan envelope or plan operation | plan artifact, active work cell, or blocker |
| `act` | action/content envelope or effect operation | admission, observation, artifact, state patch, or failure cell |
| `observe` | check or completion operation | check row, proof row, blocker, or close evidence |
| `recover` | recovery cell or failure policy | retry decision, narrowed tools, blocked state, or supersession |
| `record` | record-family state or operation | workspace file, row, fingerprint, README, index, and route evidence |
| `maintain` | index, proof, or maintenance operation | audit row, rebuilt index, alias, proof bundle, or blocker |
| `idle` | no unblocked executable candidate | new owner turn, external event, or recovered unfinished decision |

## State Policy

Prompt assembly includes a concise state fragment with purpose, context policy,
workspace policy, and failure policy. Tool exposure is still the decision's
`ToolSetView`: `act` can expose action tools, `recover` can expose a narrowed
repair tool, and other states expose no model tools unless the persisted
decision explicitly carries an action envelope and matching view. Context still
flows through discovery, scoring, deduplication, contradiction filtering,
compression, assembly, and validation. Failures write `recovery.failure` cells
before happy responses and do not silently route to `idle`.

## Authority Rule

There is no prompt-only policy and no dispatcher-only policy. Prompt rendering
and admission are projections of the same persisted decision. A model-free
operation such as `state.resolve` may settle the selected state key directly
through the runtime loop without creating or reading a bridge step. Projected
model work uses payload-defined native `work:model/*` cells that carry their
`model.call/*` operation key. Narrow model-free effects such as
`workspace.write_text` and `workspace.append_text` are carried on the persisted
decision as effect commands and path-checked by the workspace effect edge.

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
