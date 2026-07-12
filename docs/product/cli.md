# CLI

## Purpose

Define the public command boundary and truthful exit behavior.

## Commands

```text
lkjagent --data DATA send [--new] TEXT
lkjagent --data DATA run
lkjagent --data DATA status
lkjagent --data DATA doctor [--json]
lkjagent --data DATA log [--limit N] [--follow]
lkjagent --data DATA matter list
lkjagent --data DATA matter show ID
lkjagent --data DATA workbench
```

The current parser also exposes record and workspace commands. They remain
qualified by `../current-state.md` until they use the direct effect path.

## Send

`send` creates the data directory, opens the native 18-table store, allocates a
queue sequence, and commits one owner turn, canonical owner message, intake
event, open matter, and active `matter/opened` cell in one transaction. This
bounded cutover may create a new matter for every successful send. It reports the
exact matter, turn, message, and message sequence. It creates no workspace or
scaffold files.

All ordinary prose enters this direct matter loop. Substrings such as `verify`
or `run tests` do not invoke owner substring routing.

## Run

`run --once` executes at most one persisted direct decision. `run` repeats that
same native cycle with a bounded sleep. Both open only the configured workspace
when selected work reaches a workspace operation. The daemon form remains alive
across a failed cycle with bounded backoff while durable status exposes the
blocker; `--once` still reports that cycle directly. They do not construct task
or step snapshots, plans, templates, or parallel projections.

## Status Command

Status opens only the native store and reports both resolved roots, matter
lifecycle counts, the active matter, unfinished decisions, exchanges and
effects, checks readiness, and canonical conversation identities. It does not
synthesize task or queue events.

## Exit Codes

Zero means the command itself completed. It does not mean every matter closed.
Invalid input, unsafe path, unavailable required config, store conflict, or failed
operator command returns nonzero with a bounded factual diagnostic.
