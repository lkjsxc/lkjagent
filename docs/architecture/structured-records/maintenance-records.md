# Maintenance Records

## Purpose

Define idempotent background maintenance records.

## Contract

Maintenance creates Failure, Recovery, MemoryEntry, Decision, and
VerificationCheck records only when no owner task is active or recoverable and
the owner queue is empty. It may not ask the owner which transcript spans,
memory rows, or internal policy mismatches to process.

Pruning is runtime-owned unless delete, rewrite, or merge operations are
actually exposed and admitted. A prune cycle that performs no operation closes
as no-op and sets cooldown.

## Status

design, implementation pending
