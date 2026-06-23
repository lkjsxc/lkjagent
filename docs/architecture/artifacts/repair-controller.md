# Repair Controller

## Purpose

This file owns artifact repair routing after a structure, readiness, objective, or completion audit fails.

## Contract

Repair consumes weak path records and batch cursor state, then emits one bounded next action admitted by runtime
authority. The action is an audit, focused read, `artifact.next`, bounded `fs.batch_write`, one-file `fs.write`, or
blocked handoff with exact missing evidence.

## Inputs

- artifact ledger and weak paths.
- latest audit output and unsupported claims.
- batch cursor state and retry counts.
- payload, schema, parse, and repeat faults.
- authority admission view.

## Outputs

- next repair action and exact valid example.
- updated weak path retry counts.
- batch cursor advance or fallback selection.
- blocked handoff when no productive route remains.

## Invariants

- Raw giant writes are not retried after payload overflow.
- Repeated schema failure changes action shape.
- Completed paths are not repeated.
- Failed paths remain next until repaired, skipped by a recorded fallback, or blocked with evidence.
- Repair writes require re-audit before completion.

## Failure Cases

- `artifact.next` emits placeholder content when no weak paths remain.
- `fs.batch_write` repeats an invalid example after a parser fault.
- Repair marks content ready without reading changed paths.
- A completion refusal omits the repair tool authority admits.

## Verification

- batch cursor tests for resume, failed paths, and fallback.
- payload recovery tests for bounded writes.
- completion tests requiring re-audit after repair writes.

## Status

design-only for the normalized controller.
