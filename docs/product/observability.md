# Observability

## Purpose

Describe what the owner can see while the agent runs. Observation is
read-only and never perturbs the loop: looking at the agent costs it nothing.

## Surfaces

| Surface | Content |
| --- | --- |
| `lkjagent status` | Daemon state, queue depth, open task, question, error, turns, continuation epoch, checkpoint interval, continuation decision, context usage, token usage, model log |
| `lkjagent log` | Transcript events in order: messages, actions, observations, notices, queue mutations, compactions |
| `lkjagent console` | Owner screen with transcript top, pending preview, bottom status deck, and send prompt |
| `lkjagent memory` | Full-text search over distilled memory entries |
| `lkjagent graph` | Active graph case, selected packages, missing evidence, source graph summary |
| sqlite3 on the store | Read-only forensics; schema in [../architecture/memory/store.md](../architecture/memory/store.md) |

## Transcript as Truth

The transcript is the complete account of agent behavior: every action the
model took, every observation it saw, every recorded queue mutation, every
notice the harness injected, and every compaction. If something is not in
the transcript, it did not happen. Event kinds are defined in
[../architecture/memory/transcripts.md](../architecture/memory/transcripts.md).
Recovery notices are transcript events too, so a rare parse, repeat, or tool
error leaves visible evidence and the model sees the next-step instruction.

`lkjagent log` renders events compactly: one line per event with kind, turn
when present, and a bounded preview. `--limit N` keeps the newest N events,
and `--full` prints whole payloads.

## Context Usage

`lkjagent status` reports prefix size, log size, remaining headroom, and the
compaction threshold from [../architecture/context/budgets.md](../architecture/context/budgets.md).
`lkjagent status` and `lkjagent console` now also render the compact accounting
deck: `ctx=used/window percent pressure=color`, endpoint `in/out/cache/total`,
and `prefix/log/reserve/headroom`. Missing endpoint usage fields render as
`unknown`, not zero. Status and console also expose the current model handoff
path tracked in
[../architecture/observability/model-log.md](../architecture/observability/model-log.md),
and `lkjagent model-log --print` prints the synthesized Markdown snapshot.
Status also exposes continuation epoch, turns used, checkpoint interval, last
checkpoint reason, and continuation decision so budget checkpoints are visible
as autonomous progress rather than owner waits.

## Boundaries

No metrics endpoint and no log shipping. The store is local and the CLI
reads it. The console is an interactive terminal view, not a web service,
and it sizes itself from terminal rows and columns.
Anything fancier is graph-guided shell work the agent can build on demand.

## Status

partially implemented.
