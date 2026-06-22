# Contract

## Purpose

This file owns the runtime contract around model proposals. The model endpoint
suggests one action; lkjagent owns parsing, schema validation, authorization,
effects, observations, state reduction, evidence, audits, and completion.

## Pipeline

```text
RawModelText
-> ActionCandidateParser
-> ActionCandidate
-> SchemaValidator
-> NormalizedToolIntent
-> GraphAuthorization
-> EffectExecutor
-> ObservationEvent
-> PureReducer
```

## Rules

- Raw model text never causes filesystem, shell, queue, memory, Docker, or git
  effects.
- Tool schemas and hard state decide whether an action can execute.
- Weighted guards can block an otherwise legal action.
- Audit-owned evidence is written by the audit reducer.
- Completion is a runtime decision with fresh evidence, not a model claim.

## Status

implemented
