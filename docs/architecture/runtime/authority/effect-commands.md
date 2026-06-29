# Effect Commands

## Purpose

Define runtime-owned commands that execute after a persisted decision without
requiring the model to author semantic content.

## Contract

A `RuntimeEffectCommand` is emitted by a persisted decision and interpreted by
an effect adapter. It is not a model action. The command records its decision id,
input facts, expected observation shape, and resume event before any effectful
work starts.

## Command Set

- `render_prompt_frame`: persist the prompt frame for a model-call decision.
- `hard_compaction`: write pre and post compaction snapshots and resume target.
- `closed_idle_wait`: return to idle polling without an endpoint call.
- `maintenance_defer`: record why maintenance is not allowed.
- `maintenance_cooldown`: record no-op maintenance and the next eligible time.
- `blocked_handoff`: record the exact missing evidence or exhausted route.
- `status_refresh`: refresh status from the authority ledger.
- `model_log_export`: synthesize model-log output from authority records.
- `deterministic_inspection`: run a zero-content inspection selected by the
  decision, such as listing a known artifact root.
- `deterministic_doc_audit`: run a document audit when all inputs are local and
  the decision admits no semantic content.
- `deterministic_artifact_next`: derive the next artifact write contract from
  persisted facts and cursor state without asking the model to choose the tool.
- `provider_pause`: pause or defer after provider anomaly retry budget.
- `completion_close`: close only after the central completion reducer accepts.

## Interpreter Boundary

The interpreter may read files, write store rows, call provider APIs, run tool
adapters, or format CLI output. It may not choose a different mission, widen
admitted tools, satisfy evidence, hide a fault, or close a case independently.
Any stronger fact discovered by an effect becomes a new `RuntimeEvent` and a
fresh decision.

## Observation Shape

Every command records an `EffectObservation` with:

- decision id and effect command id;
- status `ok`, `error`, `deferred`, or `blocked`;
- touched paths or store rows when present;
- provider exchange id or prompt frame id when relevant;
- next event kind and payload facts;
- no invented success evidence.

## Invariants

- Hard compaction is an effect command, not `memory.save` prompt guidance.
- Closed idle and maintenance cooldown do not call the endpoint.
- Model-log and status commands read the authority ledger rather than old state
  keys when equivalent authority rows exist.
- Completion close is an effect only after completion data names no failed gate.

## Status

specified. Runtime implementations route maintenance, compaction, completion,
status work, and exact deterministic inspections through persisted decisions.
The dense network task adds effect rows and no-provider routing for document and
artifact audits, `artifact.next` contracts, blocked handoffs, idle transitions,
and close effects.
