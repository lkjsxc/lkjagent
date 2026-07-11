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

## Archive And Rebalance Compensation

Archive commits a prepared operation and immutable file preimages before its
filesystem move. Its operation group includes files, rows, aliases, audits,
search rows, generated indexes, and state effects. Rebalance still uses
metadata-only prepared rows. Archive compensation restores verified preimages
or leaves the operation prepared rather than overwriting owner bytes.

## Recovery

Startup resumes a prepared archive only when its moved target exactly matches
the intended bytes and its prior path is absent; settled retries validate bytes.
A conflict or duplicate blocks startup and leaves the operation prepared. Recovery
for rebalance, temporary files, manifests, and every writer remains open.
Archive moves use Linux no-clobber renames; unsupported hosts fail without moving bytes.

## Paths

Document IDs are stable; paths are locations. Aliases preserve moved paths.
Tombstones preserve deletion evidence. Search, relations, and index debt all
reference document IDs and revision fingerprints.
