# Import And Store Reset

## Fresh Runtime Store

Backward compatibility is not required. Replace the current mixed bridge schema
with a fresh native event and projection schema.

## Preserve Owner Files

Never delete the visible workspace during reset. Take a read-only inventory,
fingerprint managed files, and copy ambiguous old content to an import-review
area. Keep a backup path outside the active workspace until validation passes.

## Import

1. acquire the daemon lease and pause mutations;
2. take a stable workspace fingerprint inventory;
3. build a sibling database and immutable revision store;
4. scan files and identify managed record families;
5. reference ambiguous originals and write import-review diagnostics;
6. create document identities and history from current bytes;
7. rebuild search and navigation projections;
8. run integrity, foreign-key, link, size, date, and duplicate checks;
9. atomically activate the sibling database or roll back;
10. begin a fresh runtime event log and release the lease.

## Old Runtime Data

Old prompt logs, task rows, and synthetic run summaries may become test fixtures
after redaction. They do not enter the new control ledger.

## Acceptance

A new empty database rebuilt from the same workspace yields the same document
inventory, search results, navigation, and fingerprints.

Evidence captures SQLite with the online backup API or a quiesced equivalent,
then captures the workspace fingerprint manifest at the same boundary. Copying a
WAL database file alone is invalid evidence.
