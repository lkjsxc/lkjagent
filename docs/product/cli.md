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

`send` commits one owner turn, canonical owner message, initial event, matter,
obligations, and state in one transaction. It returns stable owner-turn and
message identities, not presentation text such as queue state.

All ordinary prose enters the direct matter loop. Operator slash/CLI commands may
remain deterministic. Substrings such as `verify` or `run tests` cannot divert an
ordinary edit into a terminal route.

## Run

`run` keeps cycling while eligible work or a due wake exists. A developer
bounded-cycle option may return control without changing matter state. One cycle
is never described as matter completion.

## Status Command

Status reports resolved data/workspace roots, lease, active matters, waits,
blocks, current decision, unsettled effects, and config source without secrets.
It derives lifecycle from reduced state rather than retired task rows.

## Exit Codes

Zero means the command itself completed. It does not mean every matter closed.
Invalid input, unsafe path, unavailable required config, store conflict, or failed
operator command returns nonzero with a bounded factual diagnostic.
