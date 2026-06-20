# Selection

## Purpose

Define the pure priority order that selects the active mode from runtime
facts.

## Inputs

Selection receives a plain snapshot:

- pending owner queue row count.
- active owner case flag.
- recoverable owner case flag.
- compaction required flag.
- maintenance due flag.
- maintenance active flag.
- endpoint retry pending flag.

The selector performs no IO. Store, graph, context, endpoint, and maintenance
adapters gather the snapshot before calling it.

## Priority

Pending owner rows select `OwnerTask`. A recoverable owner case selects
`Recovery`. An active owner case selects `OwnerTask`. Required compaction
selects `Compaction` only when no owner work can run. Active or due
maintenance selects `Maintenance` only when no owner work exists. Otherwise
the mode is `ClosedIdle`.

## Endpoint Retry

If endpoint retry is pending, the endpoint decision may wait without appending
turn noise. Retry state does not let maintenance override owner work.

## Evidence

Tests must cover each priority row and must use real pending queue and
maintenance facts in runtime paths. Runtime code must not hardcode pending
owner rows to zero.
