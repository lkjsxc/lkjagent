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
intended file bytes before a filesystem move. Archive compensation restores
verified preimages or leaves an operation prepared rather than overwriting owner
bytes. Rebalance stores exact file and record preimages, but exact rollback of
link history, scaffold files, search, index artifacts, and multi-move groups
remains open; recovery therefore prefers verified forward settlement.

## Recovery

Startup uses immutable preimages to settle archive and rebalance moves only when
the target exactly matches intended bytes and the prior path is absent. Settled
retries validate bytes. An exact unstarted rebalance remains prepared and blocks
startup until explicit `apply-rebalance`; that command revalidates both revisions
before moving. Missing revisions, malformed intent, changed source bytes, a
conflict, or a duplicate block without moving owner bytes. A startup projection
failure leaves matching moved bytes and the operation prepared for forward retry.
Move conflicts and post-rename sync errors also remain prepared for explicit retry.
Multi-move groups, compensated-key reuse, exact normal-apply projection rollback,
temporary-file, manifest, and broader writer recovery remain open. Archive and
rebalance moves use Linux no-clobber renames; unsupported hosts fail without moving bytes.

## Paths

Document IDs are stable; paths are locations. Aliases preserve moved paths.
Tombstones preserve deletion evidence. Search, relations, and index debt all
reference document IDs and revision fingerprints.
