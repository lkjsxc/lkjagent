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
stamped with holder id, start time, and heartbeat time. A second
`lkjagent run` against the same store refuses to start and prints the
holder unless it is the same holder replacing its prior process. A stale
lock is reclaimed with a notice event when its heartbeat is older than the
effective stale window. The effective stale window is
daemon.lock-stale-seconds, but never less than endpoint timeout plus 60
seconds.

## Startup

1. Open the store; run schema setup if tables are missing.
2. Load config from data/lkjagent.json per
   [../../operations/running.md](../../operations/running.md).
3. Load source graph definitions from lkjagent-graph.
4. Reconstruct the active graph case from graph tables when one exists.
5. Build the prefix: system prompt, registry, graph state, workspace brief,
   and memory digest ([../protocol/system-prompt.md](../protocol/system-prompt.md)).
6. If a graph case is active, append its graph resume notice as the first log
   frame; otherwise start at the queue.

Startup never replays raw transcript history into the context; graph state and
evidence carry state across restarts, which keeps restart cost flat over time.

## Shutdown

The process has no custom SIGTERM or SIGINT drain path. Container stop ends
the process. A replacement with the same holder id takes over immediately;
other replacements continue from the store after the lock heartbeat ages
past the stale window. SIGKILL loses at most one in-flight endpoint call or
tool action; the transcript and queue stay consistent because every event
write is a single transaction.

## Restart and Failure

The container supervisor restarts the daemon on nonzero exit. Endpoint
outages do not fabricate replies: the daemon appends an error event, stores
`daemon_state=error`, and tries again on later polls. Internal process loss
is recovered by stale lock reclaim plus durable queue, graph cases,
transcript, memory, config, and workspace state.

## Status

implemented.
