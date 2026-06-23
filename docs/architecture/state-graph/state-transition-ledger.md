# State Transition Ledger

## Purpose

This file owns the durable case and transition records for the unified state-transition network.

## Contract

A case enters a `CaseEnvelope` before endpoint execution. The envelope links owner objective, active node, active
mission, constraints, ledgers, authority head, prompt frame, context snapshot, verification state, maintenance state,
blocked handoff, and timestamps. Transitions append records rather than rewriting history.

## Inputs

- owner message id and objective.
- normalized objective and task family.
- hard state, active node, and active mission.
- constraints, assumptions, risks, success criteria, and evidence requirements.
- artifact, fault, authority, prompt, context, verification, maintenance, and handoff references.

## Outputs

- case envelope.
- transition records with source node, target node, event id, decision id, and guard result.
- current authority head pointer.
- status fields for CLI and prompt rendering.

## Invariants

- Every transition names the event and decision that authorized it.
- Source graph edge data is recorded as guidance, not as admission authority.
- Case status is derived from the latest ledger head and completion decision.
- Handoffs preserve missing evidence and resume route.
- Transitions retain enough data for benchmark replay.

## Failure Cases

- A case closes without a decision linked to the close event.
- A recovery route cannot be explained from stored transition data.
- Compaction resumes at a node with no record of the previous mission.
- Status output shows active mode fields that prompt rendering did not use.

## Verification

- store tests for case envelope and transition history.
- status tests for latest authority and transition fields.
- benchmark fixtures for replaying uploaded failure signatures.

## Status

design-only.
