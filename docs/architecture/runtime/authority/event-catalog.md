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

The runtime event set is closed and table-tested. String names exist only at the
store and log boundary.

Owner queue events:

- owner message queued;
- owner message delivered;
- owner message redelivered;
- owner message edited;
- owner message deleted.

Prompt and provider events:

- prompt frame rendered;
- prompt frame skipped by runtime effect;
- provider response received;
- provider endpoint error;
- provider anomaly.

Parser and admission events:

- model action parsed;
- parse fault;
- implicit envelope normalized;
- admission accepted;
- admission refused;
- stale action refused;
- repeated action refused.

Tool and graph events:

- effect started;
- observation ok;
- observation error;
- graph plan recorded;
- graph transition accepted;
- graph transition refused;
- graph evidence recorded.

Artifact and verification events:

- artifact root missing;
- artifact scaffold applied;
- artifact weak paths found;
- artifact batch cursor advanced;
- artifact audit passed;
- artifact audit failed;
- artifact drift found;
- artifact invalid root found;
- verification requested;
- verification passed;
- verification failed.

Completion, compaction, maintenance, and wait events:

- completion requested;
- completion accepted;
- completion refused;
- blocked handoff recorded;
- compaction pressure detected;
- compaction pre snapshot recorded;
- compaction completed;
- compaction post snapshot recorded;
- compaction resume requested;
- maintenance due;
- maintenance started;
- maintenance deferred;
- maintenance no-op cooldown recorded;
- maintenance completed;
- external owner question admitted;
- external owner answer delivered;
- closed idle selected.

## Source Adapters

| Adapter | Event category |
| --- | --- |
| owner queue | queued, delivered, redelivered, edited, deleted |
| prompt renderer | rendered or skipped frame |
| provider client | response, endpoint error, provider anomaly |
| parser | parsed action, parse fault, normalization |
| admission gate | accepted, refused, stale, repeated |
| dispatcher | effect started, observation ok, observation error |
| graph | plan, transition, evidence |
| artifact | root, scaffold, weak path, cursor, audit, drift |
| verifier | requested, passed, failed |
| completion reducer | requested, accepted, refused, blocked handoff |
| compaction | pressure, pre snapshot, completed, post snapshot, resume |
| maintenance | due, started, deferred, no-op cooldown, completed |
| owner wait | external question or answer |

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
