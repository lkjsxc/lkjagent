# State Transitions

## Purpose

Define pure event reduction and deterministic candidate selection.

## Reducer

```text
RuntimeSnapshot + RuntimeEvent + CurrentTime -> RuntimeState
```

The reducer validates event identity, applies cell changes, invalidates stale
evidence, derives projections, and returns a new immutable snapshot. It performs
no SQLite, filesystem, endpoint, or clock effect.

## Selector

```text
RuntimeState + Policy + CurrentTime -> RuntimeDecisionSpec
```

Candidate eligibility checks active needs, dependencies, conflicts, budgets,
wakes, source freshness, effect settlement, and check state. Stable priority,
causal sequence, and operation key break ties.

The selected spec is persisted before context compilation. It owns the exact
state selection, operation, tool fields, grammar, information needs, budgets,
recovery, checks, and exit policy.

## Initial Transition Table

| Fact | Event | Next phase | Model tools |
| --- | --- | --- | --- |
| owner turn | matter opened | orient | list, search, read |
| current target read | source need met | modify | read, edit, create |
| edit committed | revision observed | review | none |
| check failed | measured difference | modify or recover | narrowed read/edit |
| checks passed | obligations met | respond | none; final envelope |
| final persisted | close eligible | idle | none |
| parse fault | fault recorded | recover | smallest intended view |
| owner fact missing | question persisted | waiting | none |

## Guards

- A provider call requires a compilation-complete decision.
- An effect requires admission from that decision's exact tool spec.
- Review schedules native checks without model cooperation.
- Respond cannot mutate files.
- Idle requires no runnable operation, due wake, or unresolved settlement.
- A failed decision tuple cannot repeat without a material change.
