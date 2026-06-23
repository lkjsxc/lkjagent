# Maintenance Rewrite Pruning

## Purpose

This file owns idle-only semantic pruning that rewrites or deletes low-value memory rows.

## Contract

Maintenance pruning runs only when runtime authority selects idle maintenance. It deletes exact duplicates, merges
same-title high-overlap rows, or rewrites several low-signal rows into one useful row that preserves source ids. A
no-op result records cooldown and does not write a low-value memory row.

## Inputs

- queue and active case state.
- maintenance cooldown state.
- candidate memory rows with ids, titles, bodies, tags, and source ids.
- duplicate, overlap, and low-signal scores.
- authority admission view for idle maintenance.

## Outputs

- deleted duplicate ids.
- merged or rewritten memory row with source ids.
- no-op cooldown when no useful change exists.
- maintenance audit observation.

## Invariants

- Owner work, recoverable owner work, verification repair, artifact repair, and hard compaction block maintenance.
- `agent.ask` is not admitted for internal maintenance choices.
- `memory.save` writes only non-duplicate high-value lessons.
- Rewrite output keeps source ids from the rows it replaces.
- Maintenance `agent.done` cannot close owner work.

## Failure Cases

- Maintenance writes another low-value no-op row.
- Owner queue work appears before dispatch but stale maintenance still executes.
- A merge drops source ids and loses audit traceability.
- Maintenance asks the owner how to prune internal memory.

## Verification

- maintenance preemption tests before endpoint and before dispatch.
- memory pruning tests for exact duplicate deletion, high-overlap merge, rewrite, and no-op cooldown.
- completion tests proving maintenance close is separate from owner close.

## Status

partially implemented for duplicate deletion, same-title high-overlap merge, and low-signal maintenance rewrite
pruning with preserved source row ids. Runtime scheduling and no-op cooldown proof remain open.
