# Transaction Protocol

## Purpose

Define atomic, idempotent workspace changes across files and SQLite rows.

## Preparation

The decision names document IDs, current revision fingerprints, normalized
paths, intended bytes, required checks, and one idempotency key. An accepted
admission and prepared effect journal row commit before a filesystem change.

## Staging

Writes use a temporary file in the target filesystem. The effect verifies the
expected prior fingerprint, writes exact bytes, flushes the file, syncs its
directory, and atomically renames it. Multi-document changes stage a manifest
and remain one semantic operation.

## Settlement

Settlement records immutable revision bytes and SHA-256, updates the current
document revision and path, appends the runtime event and state patch, marks
dependent search or index projections dirty, runs checks, and commits the
effect observation.

## Recovery

Startup reconciles temporary files, target bytes, manifest, and effect state.
It completes, compensates, or records failure without duplicating document
identity. Owner edits are never overwritten when the prior fingerprint changed.

## Paths

Document IDs are stable; paths are locations. Aliases preserve moved paths.
Tombstones preserve deletion evidence. Search, relations, and index debt all
reference document IDs and revision fingerprints.
