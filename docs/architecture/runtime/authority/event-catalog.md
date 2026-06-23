# Event Catalog

## Purpose

This file owns the closed set of runtime authority events that can produce decisions.

## Contract

Runtime authority changes state only through named events. Effects, model output, graph suggestions, maintenance
ticks, compaction pressure, verification results, and completion requests are reduced through the same event path.

## Inputs

- owner queue and case intake observations.
- parsed model actions and parse faults.
- schema, tool, repeat, payload, endpoint, budget, context, verification, compaction, and completion faults.
- tool observations and tool errors.
- audit, evidence, verification, compaction, maintenance, and turn-budget signals.

## Outputs

- `RuntimeEvent` record with case id, event kind, payload, and timestamp assigned at the store boundary.
- one reducer call for the event.
- one persisted decision linked to the event.

## Invariants

- Unknown event kinds are rejected at the boundary.
- Event payloads carry facts only; they do not decide admission or completion.
- Tool observations become events before any next prompt is rendered.
- Context pressure emits a compaction event rather than a model-authored memory request.
- Completion requests are events, not direct close commands.

## Failure Cases

- A maintenance tick bypasses owner preemption.
- A graph complete suggestion closes the case without a completion event.
- A parse fault loops without a recorded retry and selected recovery route.
- A payload overflow retries the same giant write text.

## Verification

- event decoding tests for every event kind.
- reducer tests asserting one decision per event.
- recovery tests that record retry counts by event class.

## Status

partially implemented. The normalized event table and store API exist. Turn authority refresh records closed event
kinds for owner intake, recovery, compaction, maintenance, and turn checkpoints. Tool observations and dispatch-time
admission events remain open.
