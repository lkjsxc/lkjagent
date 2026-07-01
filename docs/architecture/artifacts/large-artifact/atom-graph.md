# Atom Graph

## Purpose

Define the durable SQLite rows that store large-artifact truth.

## Tables

| Table | Owner |
| --- | --- |
| `artifact_plans` | Objective frame, profile, root, accepted floor, and plan status. |
| `artifact_atoms` | One bounded content or assembly unit per row. |
| `artifact_atom_edges` | Dependency edges between source atoms and target atoms. |
| `artifact_write_contracts` | Active and historical exact-path write contracts. |
| `artifact_atom_events` | Write, audit, retry, weak-class, and readiness events. |
| `artifact_assembly_runs` | Deterministic assembly evidence. |
| `artifact_readiness` | Latest compact projection used by runtime and CLI surfaces. |

## Atom State

An atom is `planned`, `contracted`, `written`, `weak`, `ready`, `assembled`, or
`blocked`. Readiness is never inferred only from the last observation string.
Audit measures the file named by the atom row and records the measured count,
weak classes, and event.

## Edges

Edges make assembly and ordering explicit. A target atom is eligible only when
its dependency atoms are ready or assembled. A source atom is eligible when all
predecessors are ready and no active contract for another atom exists.

## Projection

`artifact_readiness` stores atom totals, ready count, missing count, next atom,
next path, active contract id, measured total, accepted floor, assembly pending,
readiness status, and completion blockers.

## Status

implemented.
