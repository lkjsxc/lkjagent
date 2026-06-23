# Replay Regressions

## Purpose

Define how long-running model logs become small replay fixtures that prove a
runtime failure cannot recur.

## Source Evidence

Active evidence includes `data/logs/current-model-run.md`, SQLite rows under
`data/lkjagent.sqlite3`, owner-provided LM Studio logs, and reduced fixtures in
the benchmark crate. Raw logs are diagnosis input; reduced fixtures are the
tracked regression contract.

## Reduction Steps

For each failure class, capture only the minimal fields needed to reproduce the
runtime decision:

- prompt frame summary and authority decision id when present.
- raw assistant content and provider finish reason.
- provider reasoning fields when present.
- expected parse status and fault class.
- expected recovery route.
- expected admission status.
- expected observation shown to the next model turn.

Large raw logs stay out of commits unless they are intentionally small fixtures.
The reduction note names any evidence not inspected and why.

## Required Failure Classes

The fixture suite covers these classes before the related blocker closes:

- empty model content.
- missing action block.
- unclosed action block and stop-closure repair.
- multiple action blocks.
- tool outside the active admission surface.
- payload too large.
- invalid batch-write payload.
- repeat action after refusal.
- contradictory authority frame.
- scaffold-only artifact progress.
- completion blocked by missing evidence.
- interrupted generation.

## Fixture Shape

A replay fixture stores structured JSON:

```json
{
  "name": "missing-act-block",
  "prompt_frame": { "active_mode": "Recovery", "admitted_tools": ["graph.recover"] },
  "model_output": { "content": "", "finish_reason": "stop" },
  "expected": {
    "parse_fault": "EmptyContent",
    "admission": "refused",
    "recovery_route": "NoDispatchResume"
  }
}
```

## Gate Contract

The replay gate runs without a live provider. It feeds fixture records through
the parser, authority reducer, admission gate, and recovery route table. A pass
proves the runtime refuses the old bad action and emits one valid next action
from the admitted tool surface.

## Inspection Note

Each session that inspects long-running data writes `tmp/data-inspection.md`
with existing data files, log files, SQLite tables, observed error patterns,
created fixtures, and uninspected evidence.

## Status

partially implemented. The benchmark corpus already includes uploaded-run
signatures. Full replay coverage for every listed failure class remains open.
