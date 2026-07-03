# Crash And Resume

## Purpose

Define transaction boundaries and daemon boot recovery.

## Turn Transaction

Each completed turn commits attempt, token usage, check results, events, step
updates, and task updates in one SQLite transaction. The transaction owner is
`store.turn.transaction=single`.

## Boot

At boot the daemon:

- opens the store and enables WAL plus foreign keys;
- reclaims the lock when heartbeat staleness exceeds
  `daemon.lock-stale-seconds=300`;
- loads the first non-terminal task in FIFO order from normalized rows;
- hydrates a pure in-memory snapshot only from those rows;
- resumes at engine selection.

## Loss Bound

A crash during an endpoint call can leave exchange files without rows and lose
that uncommitted model call. It cannot mark a step done or a task closed without
committed check rows.

## Failure Boundary

Resume reads only durable rows and ignores config snapshots and orphaned
exchange bodies, so a crash cannot replay stale instructions as prompt context.
