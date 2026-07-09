# Transaction Protocol

## Operation Journal

Every semantic mutation begins with a prepared operation containing:

- idempotency key from queue, decision, and operation identity;
- target path and expected prior fingerprint;
- staged path and intended new fingerprint;
- required row, index, and state effects;
- compensation or resume strategy.

## Apply

1. Validate canonical root and expected fingerprint.
2. Preserve prior bytes and intended bytes in immutable revision storage.
3. Write a sibling temporary file with restrictive mode and flush the file.
4. Atomically rename into place, then flush the parent directory entry.
5. Commit workspace document, revision, event, index debt, and effect outcome.
6. Apply derived index updates through their own idempotent operations.

## Crash Recovery

On startup, inspect prepared operations:

- staged only: validate and resume or discard safely;
- final file with intended fingerprint: commit missing rows from immutable
  intended revision bytes;
- conflicting final file: quarantine and report;
- committed rows with missing file during an unsettled effect: restore from the
  immutable intended revision or mark repair.

An external deletion observed after a settled effect becomes a debounced
owner-content tombstone and invalidates projections. Do not resurrect it as
crash recovery.

## Retry

The same idempotency key returns the prior committed result. It does not create
a second TODO, calendar entry, finance row, journal update, or transcript.

## Multi-File Artifacts

Use an operation group with semantic units. Completion occurs after every unit
and manifest fingerprint passes; partial groups remain visible and resumable.
