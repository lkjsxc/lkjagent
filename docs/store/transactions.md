# Store Transactions

## Purpose

Define atomic boundaries around intake, decisions, providers, effects, and close.

## Owner Intake

One transaction inserts owner turn, canonical owner message, runtime event,
matter, obligations, and initial state. The store derives the stable logical ID
from owner-turn identity and allocates the next positive conversation sequence
under an immediate write transaction. An exact retry, including after database
reopen, returns that identity and sequence without another row. A conflicting
reuse fails without partial intake. No task or generic event row is projected
into conversation.

## Decision Compilation

First persist immutable operation and exact state/tool/grammar/context-need/
recovery/check/exit specs with compiler state `compiling`. Compile from that row,
then atomically attach selected source refs, rendered frame, and fingerprints.
Only a compilation-complete decision may create provider intent or effect prep.

## Provider

Persist provider request intent before network I/O. A finished exchange stores
content hash/reference, usage, finish reason, anomaly, parse result, and timing.
An ambiguous sent request is not replayed after restart.

## Effect

One transaction inserts accepted admission, prepared journal, target paths,
exact prior/intended bytes and modes, stage identity, and idempotency key. Stage
and each exchange/compensation phase update durably around filesystem boundaries.

Settlement verifies descriptor-relative target state and commits observation,
workspace revision, native checks, runtime events, and decision status. Unknown
external state blocks without overwrite.

## Close

One transaction verifies required current passed checks and settled runtime
work, allocates the next conversation sequence, and commits the completion event,
final canonical message, exact receipt bytes and fingerprint, check-evidence
bindings, replacement projection, and matter lifecycle. The stable final logical
ID is derived from the completion event. An exact retry after commit or database
reopen returns the existing identity and sequence; changed bytes conflict rather
than duplicate. Message persistence failure does not repeat completed effects.

## SQLite Policy

Use foreign keys, WAL where appropriate, busy timeout, bounded transactions, and
Online Backup for evidence. Product code returns typed errors and has no panic
path.
