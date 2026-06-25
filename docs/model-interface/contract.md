# Contract

## Purpose

This file owns the runtime contract around model proposals. The model endpoint
suggests one action; lkjagent owns parsing, schema validation, authorization,
effects, observations, state reduction, evidence, audits, and completion.

## Pipeline

```text
ProviderResponse
-> ProviderAnomalyClassifier
-> RawModelText
-> ActionCandidateParser
-> ActionCandidate
-> SchemaValidator
-> NormalizedToolIntent
-> RuntimeAdmission
-> EffectExecutor
-> ObservationEvent
-> PureReducer
```

## Rules

- Raw model text never causes filesystem, shell, queue, memory, Docker, or git
  effects.
- Provider anomalies are classified before action parsing.
- Reasoning-only or empty-content provider messages never become action text.
- Tool schemas and hard state decide whether an action can execute.
- Runtime admission, not graph text, decides whether an action can execute.
- Audit-owned evidence is written by the audit reducer.
- Completion is a runtime decision with fresh evidence, not a model claim.

## Status

implemented
