# Deduplication

## Purpose

Define how the runtime prevents duplicate records, files, and memory rows.

## Checks

Duplicate detection uses these checks before writes:

- exact normalized title match.
- exact normalized content hash.
- near-duplicate title plus high content overlap.
- same tags plus same owning task.
- same artifact root plus same section role.
- same artifact kind plus same normalized title.
- same owner objective hash.

## Decisions

A candidate becomes Insert, SkipDuplicate, UpdateExisting, MergeWith, or
RepairExisting. The decision is recorded as evidence and returned to the model
as a bounded observation. Silent duplicate writes are forbidden.

## Memory Identity

Memory identity uses kind, title slug, sorted tags key, and content hash. A
duplicate memory save returns the existing id or a skip notice; it does not
create another equivalent row. Maintenance-generated rows dedupe aggressively.

## Status

partially implemented; exact memory duplicate skips and exact duplicate prune
exist. Near-duplicate merge, artifact adoption, and repair decisions remain
open.
