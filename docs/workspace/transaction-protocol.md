# Transaction Protocol

## Purpose

Define atomic, idempotent workspace changes across files and SQLite rows.

## Preparation

The decision names document IDs, current revision fingerprints, normalized
paths, intended bytes, required checks, and one idempotency key. Rebalance keys
bind record identity, old path, new path, and source fingerprint. An accepted
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

Archive and rebalance commit a prepared operation with immutable prior and
intended file bytes before a filesystem move. Their operation groups include
files, rows, aliases, audits, search rows, generated indexes, and state effects.
Archive compensation restores verified preimages or leaves an operation prepared
rather than overwriting owner bytes. Rebalance multi-move compensation remains partial.

## Recovery

Startup uses immutable preimages to settle archive and rebalance moves only when
the target exactly matches intended bytes and the prior path is absent. Settled
retries validate bytes. A conflict or duplicate blocks startup and leaves the
operation prepared. A rebalance projection failure compensates before settlement.
Operations without move bytes, unstarted, multi-move,
temporary-file, manifest, and broader writer recovery remain open. Archive and
rebalance moves use Linux no-clobber renames; unsupported hosts fail without moving bytes.

## Paths

Document IDs are stable; paths are locations. Aliases preserve moved paths.
Tombstones preserve deletion evidence. Search, relations, and index debt all
reference document IDs and revision fingerprints.
