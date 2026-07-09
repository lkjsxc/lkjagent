# Authority Model

## Canonical Function

The pure core implements:

    RuntimeSnapshot + RuntimeEvent + CurrentTime -> RuntimeState
    RuntimeState + Policy + CurrentTime -> RuntimeDecision

The effects edge implements:

    RuntimeDecision -> EffectResult
    EffectResult -> RuntimeEvent

## Required Removal

TaskSnapshot, task rows, step rows, fixed templates, and bridge projections may
be read only by an offline fixture converter during development. Production
selection, prompt rendering, effect dispatch, recovery, and completion must not
read them.

## Decision Contents

Every persisted decision includes:

- selected state keys and dependency edges;
- operation identity and idempotency key;
- derived prompt state;
- context plan and fingerprint;
- tool view and fingerprint;
- expected envelope grammar;
- input and output budgets;
- required observations and checks;
- fault-specific recovery policy;
- exit predicate and next wake time.

## Durable Boundaries

Use two database transactions around an external effect:

1. commit the decision, accepted admission, operation lease, and prepared effect
   intent before execution;
2. perform the staged or external effect;
3. commit effect outcome, observation, state patch, checks, outbox message, and
   decision settlement after the effect is durably resolved.

Crash recovery reads the prepared intent and reconciles actual external state.
Harness-selected native effects receive a typed harness admission through the
same validator. Rejected admissions commit without an effect intent.

## Prohibited

No dispatcher-only registry, prompt-only policy, model finish shortcut, or
synthetic idle task may bypass this decision.
