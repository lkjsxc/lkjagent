# State Vector

## Purpose

The state vector owns storage and pure updates for weighted tracks. Reducers
change the vector from events; tools and prompts read it but do not mutate it.

## Reducer Contract

```text
CaseEvent + StateVector -> StateVector
StateVector + ToolIntent -> authorization pressure
StateVector + PromptMode -> required context slices
StateVector + AuditResult -> evidence and guard updates
```

## Freshness

Each update records the source event and the last updated event. A completion
attempt reads only fresh gates: objective contract, documentation contract,
relation audit, mock-content audit, model-name audit, and verification.

## Context Slices

Dominant tracks select compact slices:

- parse recovery: grammar, tool schemas, last parser faults.
- documentation contract: owner objective and topic roles.
- structure connectivity: relation graph and backlinks.
- model-specific naming: sanitizer report and raw fixture pointer.
- context pressure: snapshot and consistency checklist.

## Status

implemented
