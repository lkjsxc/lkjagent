# Crash And Resume

## Purpose

Define transaction boundaries and deterministic recovery from durable rows.

## Event Transaction

Appending a runtime event, validating its transition, writing the state patch,
and recording old and new fingerprints is one database transaction. A partial
event or partial state patch is impossible.

## Effect Transactions

Before an external or workspace effect, commit the selected decision, accepted
admission, operation lease, and prepared journal intent. Perform the staged
effect outside that transaction. Then commit outcome, observation, event and
state patch, current checks, outbox message, and settlement.

Rejected admissions commit their reason without a journal row.

## Startup

One daemon obtains the write lease, enables foreign keys, WAL, busy timeout,
and full synchronous settlement, then processes:

1. prepared or applying effects;
2. interrupted endpoint decisions;
3. stale derived projections;
4. due wake conditions;
5. pending owner turns.

Recovery compares intended, prior, actual, and outcome fingerprints. It commits
recovered, compensated, or failed state and exactly one settling observation.
It never repeats a committed semantic effect.

## Evidence Snapshot

Acceptance captures SQLite with Online Backup at a quiesced read boundary. A
workspace manifest from that boundary binds current document IDs, revisions,
paths, and byte fingerprints.
