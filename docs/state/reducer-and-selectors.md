# Reducer And Selectors

## Purpose

Define pure state transitions and deterministic turn selection.

## Event Model

Events are append-only facts from queue intake, owner answers, parser results,
tool admissions, effects, observations, checks, compaction, recovery, and
completion attempts. Each event has a kind key, payload schema, payload JSON,
source ref, created time, and optional decision id.

## Reducer

The reducer is pure:

```text
RuntimeSnapshot + RuntimeEvent -> StatePatch
```

A patch inserts, updates, suppresses, resolves, or blocks state cells. It may
also add or suppress state edges. The store commits the event and patch in one
transaction. The reducer never reads files, opens SQLite, calls the endpoint, or
asks the wall clock.

## Selectors

Selectors read the hydrated state vector and compact state-edge evidence, create
bounded `SelectorCandidate` values, filter blocked candidates, sort them
deterministically, and persist the winning `RuntimeDecision`. Selection prefers
recovery, owner intake or answers, safety resolution, due scheduled work,
admitted effects, stale checks, model calls, compaction, maintenance, then idle.

## Fingerprints

State-vector and snapshot fingerprints are canonical over stable bytes such as
canonical JSON. Rust debug formatting is not a persisted fingerprint format.

## Failure This Prevents

The daemon can replay events into the same state, and tests can prove decisions
without filesystem, endpoint, or SQLite side effects.
