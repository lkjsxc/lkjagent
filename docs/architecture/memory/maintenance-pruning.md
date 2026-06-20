# Maintenance Pruning

## Purpose

Define memory pruning during idle maintenance.

## Admission

Pruning runs only in `Maintenance` active mode and only when no owner row is
pending, no owner case is active, and no owner case is recoverable.

## Operations

Exact duplicate pruning deletes real rows and their search rows. Same-title
high-overlap pruning merges source rows into the oldest matching row, records
source row IDs, updates the search row, and deletes superseded rows. Rewrite
pruning remains open and must name changed row IDs and source row IDs when it
is implemented.

## No-Op

If pruning finds nothing useful to change, it sets cooldown and writes no
memory row claiming a maintenance lesson. Memory rows record what happened,
not what the agent wishes happened.

## Refusals

Maintenance cannot ask the owner which row to inspect, merge, or delete when
internal search, ranking, or a safe no-op close can proceed.
