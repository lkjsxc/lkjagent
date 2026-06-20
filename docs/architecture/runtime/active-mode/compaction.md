# Compaction

## Purpose

Define hard compaction as a runtime-owned action.

## Contract

When context pressure requires compaction, the runtime snapshots recoverable
state and rewrites context without asking the model to run a tool. The model
may later write optional lessons during maintenance, but hard compaction never
depends on `memory.save`.

## Snapshot

The snapshot preserves active graph case, active mode, recovery state, fault
counters, last failed action fingerprint, artifact root and gaps, evidence
ledger, touched paths, token accounting, queue preemption facts, last
successful action, and any blocked handoff.

## Policy

Compaction renders no model tool policy. Dispatch refuses model-authored
compaction completion and points to runtime state instead of a blocked tool.
