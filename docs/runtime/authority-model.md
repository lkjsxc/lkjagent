# Authority Model

## Purpose

Define the only production control path from durable events to effects.

## Canonical Functions

The pure core implements:

```text
RuntimeSnapshot + RuntimeEvent + CurrentTime -> RuntimeState
RuntimeState + Policy + CurrentTime -> RuntimeDecision
```

The effects edge implements:

```text
RuntimeDecision -> EffectResult
EffectResult -> RuntimeEvent
```

Events and their state patches commit atomically. A transition records the
source event, old and new fingerprints, guard, and evidence. Production calls
the transition validator before commit.

## Decision Contents

Every persisted decision records:

- selected state cells and dependency edges;
- operation identity and unique idempotency key;
- derived prompt state;
- context, tool-view, grammar, budget, recovery, and exit fingerprints;
- required observations and checks;
- next wake time.

The decision commits before prompt compilation or effect execution. Prompt
cards, tool views, admissions, effects, recovery, and completion all reference
that row.

## Authority Limits

Fresh production selection, prompt rendering, effect dispatch, recovery, and
completion read no `TaskSnapshot`, task, step, template, plan-family, or bridge
projection. An offline fixture converter may read retired data while extracting
regressions, but it cannot run in the daemon.

No dispatcher-only registry, prompt-only policy, model finish shortcut, or
synthetic idle object may bypass a persisted decision.
