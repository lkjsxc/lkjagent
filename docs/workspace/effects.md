# Workspace Effects

## Purpose

Define revision-safe file operations and recoverable settlement.

## Initial Operations

- List one bounded directory.
- Search bounded UTF-8 text below one path.
- Read numbered bounded lines and return a SHA-256 revision.
- Create one absent UTF-8 file without overwrite.
- Replace one exact old-text occurrence in an observed revision.

Delete, move, whole-file overwrite, and shell are not initial model tools.

## Edit Admission

An exact edit requires a current read observation for the same normalized path.
Admission rejects stale bytes, zero or multiple matches, unchanged replacement,
oversized text, unsafe paths, symlinks, special files, and undeclared collateral.
The model does not echo a fingerprint; the decision and observation bind it.

## Preparation

One SQLite transaction stores admission, idempotency, target path, exact
prior/intended bytes, expected/intended mode, deterministic stage name, and
prepared journal state. A nested create validates absence from the nearest
opened parent without opening a nonexistent directory. Its file target is first;
every absent parent follows as an exact `mkdir` target. Existing owner
directories are not targets.

After preparation, a narrow Linux `mkdirat` edge traverses no-follow, rejects
file, symlink, special, and case collisions, creates only declared parents, and
fsyncs each new directory and its parent. It does not use `create_dir_all`.
Owner bytes or a racing collision win. Declared directories left beside an
unfinished journal remain in place, and startup exposes that unfinished effect;
this slice does not claim fuller directory recovery.

## Replacement

1. Stage intended bytes with intended mode in the target directory and fsync.
2. Persist the stage identity and exchange-ready phase.
3. Reopen and validate current target bytes and mode.
4. Persist exchanging and atomically exchange target and stage.
5. Inspect captured old bytes and mode.
6. Settle only when both equal the observed preimage.

On mismatch, persist compensating. Reverse exchange must first preserve any
newer post-exchange owner edit. After reverse exchange, inspect the newly
captured target; if it was not intended, exchange again to restore that newer
value and block. Every exchange boundary has a durable phase.

## Recovery

Startup classifies target and stage `(bytes, mode)` pairs using journal phase.
Prior target plus intended stage means exchange did not occur. Intended target
plus expected captured preimage permits settlement. A third value during
compensation is the newer owner value and wins. Unknown states block without
overwrite.

Internal temp or quarantine files are removed only after verified settlement or
compensation and directory sync. Fault tests interrupt every phase and prove
latest owner bytes, mode, journal state, and no residue.

## Observation And Checks

Settlement rereads the target through the same path service, stores immutable
revision and receipt, emits one bounded observation, and settles the decision.
First journal write creates a managed document. A same-path write may replace it
only when current filesystem bytes equal the native current revision; settlement
creates a child revision. An unmanaged collision or stale owner bytes block.
Exact identical journal preparation is idempotent even after settlement;
changed reuse conflicts.

The app check reducer consumes only the committed successful observation. It
rereads the target through the opened descriptor capability, compares bytes and
mode with the first effect target and revision, checks content and all allowed
file or mkdir targets, then durably reduces obligations. Journals add a
structural lineage/path/date/token/nonplaceholder check. Repeating at the same
source revision adds no rows. A failed or stale check cannot become success
prose, and final receipts include checked path parameters and revision.
