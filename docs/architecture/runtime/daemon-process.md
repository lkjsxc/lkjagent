# Daemon Process

## Purpose

Specify the process around the loop: startup, the single-instance rule,
shutdown, and restart behavior. Owner-visible lifecycle is
[../../product/daemon.md](../../product/daemon.md).

## Process Model

`lkjagent run` is one foreground process inside the container. It owns:

- the store connection (WAL mode, per
  [../memory/store.md](../memory/store.md)),
- the endpoint client,
- the loop, executed turn by turn on a single thread of control.

There is no worker pool and no async fan-out; the endpoint call is the only
long wait, and blocking on it is correct because nothing else may proceed.

## Single Instance

The daemon takes an exclusive lock row in the store state table at startup,
stamped with process id and start time. A second `lkjagent run` against the
same store refuses to start and prints the holder. A stale lock (holder pid
dead) is reclaimed with a notice event.

## Startup

1. Open the store; run schema setup if tables are missing.
2. Load config from data/lkjagent.toml per
   [../../operations/running.md](../../operations/running.md).
3. Index the skill library ([../skills/loading.md](../skills/loading.md)).
4. Build the prefix: system prompt, workspace brief, memory digest, skill
   index ([../protocol/system-prompt.md](../protocol/system-prompt.md)).
5. If the state table records an open task, append its latest task summary
   as the first log frame; otherwise start at the queue.

Startup never replays raw transcript history into the context; summaries
carry state across restarts, which keeps restart cost flat over time.

## Shutdown

SIGTERM and SIGINT request stop; the daemon finishes the in-flight turn,
writes its observation, releases the lock row, and exits 0. SIGKILL loses at
most one in-flight turn; the transcript and queue stay consistent because
every event write is a single transaction.

## Restart and Failure

The container supervisor restarts the daemon on nonzero exit. Before exiting
on an internal error, the daemon appends an error notice event so the
failure is part of the transcript, honoring
[../../agent/honest-state.md](../../agent/honest-state.md). Endpoint outages
do not exit the process: the loop retries with capped exponential backoff
and surfaces the condition in `lkjagent status`.

## Status

design-only.
