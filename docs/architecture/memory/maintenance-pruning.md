# Maintenance Pruning

## Purpose

Define memory pruning during idle maintenance.

## Admission

Pruning runs only in `Maintenance` active mode and only when no owner row is
pending, no owner case is active, no owner case is recoverable, and hard
compaction is not required.

Owner work preempts maintenance before the next endpoint turn. A stale
maintenance action is not allowed to claim pruning after owner work changes the
active mode.

## Operations

Exact duplicate pruning deletes real rows and their search rows. Same-title
high-overlap pruning merges source rows into the oldest matching row, records
source row IDs, updates the search row, and deletes superseded rows.

Rewrite pruning removes or rewrites repeated low-value maintenance lessons. It
is implemented only when the maintenance output names changed row IDs, source
row IDs, and the operation that changed them.

## No-Op

If pruning finds nothing useful to change, it sets cooldown and writes no
memory row claiming a maintenance lesson. Memory rows record actual effects,
not intentions.

## Refusals

Maintenance cannot ask the owner which row to inspect, merge, delete, or
rewrite when internal search, ranking, or a safe no-op close can proceed.

## Status

partially implemented. Exact duplicate deletion, same-title high-overlap merge,
and no-op cooldown exist. Rewrite pruning and stale-action preemption remain
open.
