# Daemon

## Purpose

Define how the lkjagent daemon works, waits, records, and resumes.

## Lifecycle

`lkjagent run` starts one foreground daemon for one data directory. The daemon
opens the SQLite store, claims the heartbeat lease, loads flat configuration,
and enters the turn cycle.

Each cycle has one active concern:

1. deliver due owner turns;
2. classify the turn as answer, matter continuation, direct record, artifact,
   inspection, or system operation;
3. hydrate the current state graph and workspace evidence;
4. persist one `RuntimeDecision` before prompt rendering or deterministic
   effect dispatch;
5. render at most one XML-like prompt frame;
6. call the endpoint only when the decision needs model output;
7. parse the selected envelope or action grammar;
8. apply admitted deterministic effects;
9. evaluate checks and commit events, observations, state patches, workspace
   fingerprints, and token usage.

At most one endpoint call happens in a cycle. Verify-only or record-only work
consumes no model call.

## Matter States

- `open`: the matter has runnable decisions or stale evidence to refresh.
- `active`: a decision is being prepared, called, or applied.
- `waiting`: one concrete owner answer is needed.
- `blocked`: the matter exhausted a budget or safety guard and wrote evidence.
- `dormant`: no immediate operation is selected, but the matter remains useful.
- `closed`: completion checks passed and a report was recorded.
- `archived`: the matter is hidden from normal active views but preserved.

Blocked, closed, and archived states are terminal for automatic progress. A
later owner turn may create a new matter or explicitly reopen through a checked
state transition.

## Idle

No runnable matter and no pending owner turn means idle. Idle updates heartbeat
data and sleeps on the queue poll interval. It does not call the endpoint,
rewrite memory, inspect files, or self-assign work.

## Waiting

A waiting matter parks like idle, but status prints the pending question. The
next owner turn is attached as the answer unless the owner asks for a separate
new matter. Answer routing is recorded as an event and relation edge.

## Record-Only Turns

Record-like owner turns may bypass the endpoint. The deterministic router writes
or updates the workspace record, refreshes record metadata and indexes, appends
state events, and reports the path and fingerprint. If recording would be
harmful or ambiguous, the daemon asks at most one clarification.

## Crash Resume

Every committed turn is durable in SQLite and workspace files. On boot the
daemon reclaims the lock if needed, reads active matter and decision rows, and
continues from the persisted decision. A crash mid-call can lose one
uncommitted endpoint response; it cannot create a false recording or completion.
