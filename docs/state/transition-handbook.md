# Transition Handbook

## Purpose

This file owns the documentation growth transition table. It prevents a vague
request from jumping directly to a broad scaffold or premature completion.

## Documentation Flow

```text
queued -> admitted -> intake -> objective-contract -> topic-contract
-> seed-structure -> seed-audit -> expansion-plan -> local-expansion
-> structure-audit -> relation-pass -> semantic-audit -> repairing
-> verifying -> complete
```

## Repair Edges

- `seed-audit` with actionable failures moves to `repairing`.
- `semantic-audit` with quality failures moves to `repairing`.
- parse faults, repeated actions, and blocked tools move to `recovering`.
- high context pressure moves to `context-compacting` before mutation.
- a new owner task moves to intake or scheduling before mutation.

## Growth Invariants

- Start with a small connected semantic seed.
- Expand one local neighborhood at a time.
- Update README files and relation pages in the same batch.
- Run topology and semantic gates after each growth batch.
- Completion is blocked by topology-only evidence.

## Status

implemented
