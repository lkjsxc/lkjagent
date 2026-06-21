# Reducer

## Purpose

Define the pure reducer that converts durable runtime state and one event into
one explainable runtime decision.

## Contract

```text
RuntimeSnapshot + RuntimeEvent -> RuntimeDecision
```

`RuntimeSnapshot` contains daemon state, queue state, active case, active
mission, graph state, evidence ledger, artifact ledger, recovery ladder,
context budget, compaction state, maintenance state, last endpoint turn, last
tool attempt, and last tool result.

`RuntimeEvent` includes owner queue changes, parsed endpoint actions, parse
faults, tool success, tool failure, verification results, context pressure,
maintenance ticks, completion requests, and queue non-empty events.

`RuntimeDecision` is one of: execute tool, ask endpoint, refuse action, start
or continue recovery, start compaction, start maintenance, start verification,
close case, or block completion with a partial handoff.

## Invariants

- The reducer must not read files, SQLite, endpoint state, shell state, clock,
  or environment.
- Dispatch, endpoint calls, compaction writes, maintenance writes, and case
  closure happen after the decision.
- The decision must be serializable for status, logs, and GPT handoff output.

## Failure Cases

- Reducer logic calls an effect or consults current time directly.
- Two decisions are emitted for one event.
- A decision omits missing evidence or next valid action.
- Completion and dispatch use different admission gates.

## Verification

Unit tests cover every event class and assert that decisions are pure data.
Integration tests assert dispatch uses the emitted admission result unchanged.

## Related Files

- [missions.md](missions.md)
- [tool-admission.md](tool-admission.md)
- [completion.md](completion.md)
