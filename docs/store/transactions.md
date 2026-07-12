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

## Direct Tools And Faults

After a compiled decision, admitted list, search, and read calls settle in one
immediate transaction without filesystem mutation. The transaction inserts the
non-effectful admission, bounded observation, causal event, settled decision,
and active source cell containing the source revision and bytes reference. An
exact retry, including after reopen, is a no-op; changed reuse conflicts.

Malformed, hidden, or stale model output is rejected in one immediate
transaction. It writes a typed recovery cell and causal fault event and marks
the decision failed. Parse faults never create admission or effect rows. Exact
fault retries are no-ops and changed retries conflict.

## Effect

One transaction inserts accepted admission, prepared journal, target paths,
exact prior/intended bytes and modes, stage identity, and idempotency key. The
exact create/edit API also derives required open `workspace-bytes`, `content`,
and `collateral` obligations from the admitted target bytes and prior/intended
revision identity. It never inserts passing checks. Exact retries are no-ops;
changed reuse conflicts. Stage and each exchange/compensation phase update
durably around filesystem boundaries.

Settlement verifies descriptor-relative target state and commits observation,
workspace revision, native checks, runtime events, and decision status. Unknown
external state blocks without overwrite.

## Restart Projection

One native query returns the current non-closed matter, active cells, unfinished
decisions, provider exchanges and effects, plus required-current-passed-check
readiness. The projection reads only the 18 native authority tables and is safe
to repeat after reopen.

## Close

One transaction verifies required current passed checks and settled runtime
work, allocates the next conversation sequence, and commits the completion event,
final canonical message, receipt bytes and fingerprint computed from ordered
current check evidence, replacement projection, and matter lifecycle. The stable final logical
ID is derived from the completion event. An exact retry after commit or database
reopen returns the existing identity and sequence; changed bytes conflict rather
than duplicate. Message persistence failure does not repeat completed effects.

## SQLite Policy

Use foreign keys, WAL where appropriate, busy timeout, bounded transactions, and
Online Backup for evidence. Product code returns typed errors and has no panic
path.
