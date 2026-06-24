# Event Catalog

## Purpose

This file owns the closed set of runtime authority events that can produce
decisions.

## Contract

Runtime authority changes state only through named events. Effects, model
output, graph suggestions, maintenance ticks, compaction pressure, verification
results, and completion requests are reduced through the same event path. One
event emits one persisted decision.

## Event Set

The runtime event set is closed and table-tested:

- owner message received;
- queue changed;
- case opened;
- case resumed;
- prompt frame rendered;
- endpoint call requested;
- endpoint response received;
- endpoint fault;
- model action parsed;
- parse fault;
- schema fault;
- admission requested;
- admission refused;
- tool started;
- tool succeeded;
- tool failed;
- repeat action detected;
- payload overflow detected;
- evidence added;
- artifact planned;
- artifact applied;
- artifact audited;
- artifact weak path found;
- verification requested;
- verification passed;
- verification failed;
- completion requested;
- completion blocked;
- case closed;
- context pressure detected;
- compaction started;
- compaction completed;
- maintenance tick;
- maintenance started;
- maintenance no-op;
- maintenance completed;
- turn-budget checkpoint;
- turn-budget exhausted;
- owner input required;
- blocked handoff recorded.

## Inputs

- owner queue and case intake observations.
- parsed model actions and parse faults.
- schema, admission, repeat, tool, payload, endpoint, budget, context,
  verification, compaction, maintenance, and completion faults.
- tool observations and tool errors.
- audit, evidence, verification, compaction, maintenance, and turn-budget
  signals.

## Outputs

- `RuntimeEvent` record with case id, event kind, payload, and timestamp
  assigned at the store boundary.
- one reducer call for the event.
- one persisted decision linked to the event.

## Invariants

- Unknown event kinds are rejected at the boundary.
- Event payloads carry facts only; they do not decide admission or completion.
- Tool observations become events before any next prompt is rendered.
- Context pressure emits a compaction event rather than a model-authored memory
  request.
- Completion requests are events, not direct close commands.
- Endpoint faults and endpoint responses are both event inputs so parser and
  admission recovery share the same decision stream.

## Failure Cases

- A maintenance tick bypasses owner preemption.
- A graph complete suggestion closes the case without a completion event.
- A parse fault loops without a recorded retry and selected recovery route.
- A payload overflow retries the same giant write text.
- An endpoint response is parsed and dispatched without a persisted decision.

## Verification

- event decoding tests for every event kind.
- reducer tests asserting one decision per event.
- recovery tests that record retry counts by event class.
- replay tests that feed provider records through parse, admission, and step
  logic without a live endpoint.

## Status

partially implemented. The normalized event table and store API exist. Turn
authority refresh records closed event kinds for owner intake, recovery,
compaction, maintenance, and turn checkpoints. Endpoint response, parse,
dispatch-time admission, effect observation, and close-path event coverage
remain open.
