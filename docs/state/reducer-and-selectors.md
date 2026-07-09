# Reducer And Selectors

## Purpose

Define pure event reduction, transition validation, and decision selection.

## Reducer

The canonical reduction is:

```text
RuntimeSnapshot + RuntimeEvent + CurrentTime -> RuntimeState
```

It appends or supersedes cells, updates typed edges, invalidates dependent
checks and projections, advances causal sequence, and emits a transition record.
It performs no I/O and reads no wall clock except the supplied `CurrentTime`.

## Transition Validator

Every transition names source event, prior fingerprint, next fingerprint,
guard, and evidence. Guards reject terminal reopening without an owner event,
completion with unsatisfied obligations, recovery without a failure lineage,
waiting without a wake condition, effect readiness without admission, and
current checks whose source fingerprints changed.

Production calls the validator before the event and patch commit atomically.

## Selector

The canonical selection is:

```text
RuntimeState + Policy + CurrentTime -> RuntimeDecision
```

Candidate construction follows current cells and operation edges. Eligibility
checks dependencies, conflicts, budgets, admissions, cooldowns, context, and
wake time. Stable priority, causal sequence, and operation ID break ties.

The decision persists selected cells and edges, operation and idempotency,
prompt state, all compiler fingerprints, required observations and checks,
recovery policy, exit predicate, and next wake time before any prompt or effect.
