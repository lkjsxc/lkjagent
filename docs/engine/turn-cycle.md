# Turn Cycle

## Purpose

Map one daemon cycle to the state-ledger runtime loop.

## Cycle

- Intake records owner messages as queue and event rows.
- Hydration builds a `RuntimeSnapshot` from cases, events, state cells, context,
  checks, artifacts, decisions, and config.
- Selection calls pure selectors to choose one persisted `RuntimeDecision`.
- Rendering builds a `PromptFrame` only from that decision and selected context.
- Calling sends at most one endpoint request when the decision needs a model.
- Parsing expects the envelope and tool view from the decision.
- Admission validates model actions against the persisted `ToolSetView`.
- Effects run only after admission or model-free decision selection.
- Observations become runtime events.
- Checking evaluates selected predicates from [completion.md](completion.md).
- Persistence commits events, state patches, checks, usage, observations,
  admissions, and decision settlement together.

## Model-Free Turns

Queue intake, deterministic checks, file effects, compaction, recovery,
completion attempts, and idle waits may execute without endpoint access when the
decision selects an effect command.

## Ordering Guarantees

Queue rows do not interrupt a turn. Tool and shell effects run with timeouts from
their descriptors. If an effect fails before commit, the turn records an honest
error path and does not mark the plan item done or the case closed. A crash
resumes from the last committed decision and rows.

## Failure This Prevents

The cycle has one decision seam and one persistent writer. Prompt grammar, tool
admission, and effect dispatch cannot disagree for a turn.
