# Selection

## Purpose

Define the pure priority order that selects the active mode from runtime facts.

## Inputs

Selection receives a plain snapshot:

- pending owner queue row count.
- active owner case flag.
- recoverable owner case flag.
- hard compaction required flag.
- maintenance due flag.
- maintenance active flag.
- endpoint retry pending flag.

The selector performs no IO. Store, graph, context, endpoint, and maintenance
adapters gather the snapshot before calling it.

## Priority

The selector returns exactly one mode:

1. `Compaction` when hard context pressure or an unfinished compaction cycle is present.
2. `OwnerTask` when pending owner rows exist.
3. `Recovery` when an owner case has a recoverable fault.
4. `OwnerTask` when an owner case is active and not closed.
5. `Maintenance` when maintenance is active or due and no owner work exists.
6. `ClosedIdle` otherwise.

Hard compaction is first because the runtime must preserve resumability before
it admits more owner, recovery, or maintenance work. The compaction action is
runtime-owned and renders no model tool surface.

## Endpoint Retry

Endpoint retry state changes the endpoint decision, not mode priority. Retry
state never lets maintenance override queued, active, or recoverable owner work.

## Evidence

Tests cover each priority row with real pending queue and maintenance facts in
runtime paths. Runtime code must not hardcode pending owner rows to zero.

## Status

partially implemented. Pure selection exists. Runtime paths still need complete
pre-dispatch authority refresh and contradiction repair coverage.
