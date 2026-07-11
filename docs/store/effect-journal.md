# Effect Journal

## Purpose

Define admission, execution, settlement, and crash recovery for every effect.

## Admission

Every external or workspace effect has exactly one accepted model or harness
admission. This includes harness-run shell command checks, which prepare before
`/bin/sh` starts. Admissions are unique by decision and action ordinal. A
rejected admission records a typed reason and creates no effect row.

## Journal States

An effect has a unique non-null admission and idempotency key. The binding
`decision_id`, command ordinal, admission ID, and journal ID is immutable. Its
durable state is `prepared`, `applying`, `committed`, `recovered`,
`compensated`, or `failed`. Intended, prior, and outcome fingerprints make
reconciliation deterministic.

Exactly one immutable observation settles every attempted effect. The
observation records attempt outcome, bounded content reference, fingerprint,
and contamination class.

## Two Transactions

The first transaction commits the decision, accepted admission, operation
lease, and prepared effect before execution. The effect edge then stages or
performs the external change. The second transaction commits the outcome,
observation, state event and patch, checks, outbox message, and settlement.

## Recovery

Startup reads prepared or applying effects and compares actual external state
with intended and prior fingerprints. An unresolved applying row is recovery
work, never permission to replay blindly. Applying non-file effects, including
shell checks, fail rather than execute again. Recovery commits a terminal
observation without repeating a settled semantic effect.
