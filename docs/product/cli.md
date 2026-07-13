# CLI

## Purpose

Define the public command boundary and truthful exit behavior.

## Commands

```text
lkjagent help
lkjagent --data DATA send [--new] TEXT
lkjagent --data DATA run [--once]
lkjagent --data DATA status
lkjagent --data DATA doctor [--json]
```

No other command names are public. Unknown commands and unsupported arguments
fail during argument parsing, before a data directory, SQLite database, or
workspace is created.

## Send

`send` creates the data directory, opens the native 18-table store, allocates a
queue sequence, and commits one owner turn, canonical owner message, intake
event, open matter, and active `matter/opened` cell in one transaction. Without
`--new`, an owner message resumes the oldest budget-blocked matter atomically,
suppresses its block cell, and updates its objective. `--new` always creates a
separate matter. Send reports exact matter, turn, message, sequence, and resume
status. It creates no workspace or scaffold files.

All ordinary prose enters this direct matter loop. Substrings such as `verify`
or `run tests` do not invoke owner substring routing.

## Run

`run --once` executes at most one persisted direct decision. `run` repeats that
same native cycle with a bounded sleep. Both open only the configured workspace
when selected work reaches a workspace operation. The daemon form remains alive
across a failed cycle with bounded backoff while durable status exposes the
blocker; `--once` reports that cycle directly. They do not construct task or
step snapshots, plans, templates, or parallel projections.

## Status Command

Status opens only the native store and reports both resolved roots, matter
lifecycle counts, the active matter, unfinished decisions, exchanges and
effects, checks readiness, and canonical conversation identities. It does not
synthesize task or queue events.

## Doctor Command

Doctor opens only the native 18-table store, validates it, and reports resolved
roots, workspace presence, endpoint source labels, matter and unfinished-decision
counts, and configured prompt/campaign bounds. It creates no workspace and never
prints credential values.

## Help Command

Help prints only the five public command shapes. It creates no data directory,
database, or workspace.

## Exit Codes

Zero means the command itself completed. It does not mean every matter closed.
Invalid input, unsafe path, unavailable required config, store conflict, or a
failed operator command returns nonzero with a bounded factual diagnostic.
