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

## Transition Guard

The pure transition guard models generic node states such as proposed, admitted,
ready, active, waiting, blocked, recovering, verifying, succeeded, failed,
superseded, and archived. It rejects illegal state steps, terminal-state reopen
attempts, progress while active blocking or dependency edges remain, and success,
failure, or supersession without evidence refs. The guard returns data for row
commit callers; it does not execute effects or replace persisted decisions.

## Selectors

Selectors read the hydrated state vector and compact state-edge evidence, create
bounded `SelectorCandidate` values, filter candidates blocked by active
`blocks` edges, sort them deterministically, and persist only the winning
`RuntimeDecision`. The first implemented candidate tiers are owner intake,
owner answer, recovery, effects, model calls, checks, completion, workspace
record families, payload-defined custom operations, cooldown suppression, and
idle. A state cell with an `operation_key` payload can become a candidate without
adding a central enum branch. Candidate rows are not a second control plane.

## Fingerprints

State-vector and snapshot fingerprints are canonical over stable bytes such as
canonical JSON. Candidate ordering uses tier, cell priority, optional
`deadline_at`, and state key label. Rust debug formatting is not a persisted
fingerprint format.

## Failure This Prevents

The daemon can replay events into the same state, and tests can prove decisions
without filesystem, endpoint, or SQLite side effects.
