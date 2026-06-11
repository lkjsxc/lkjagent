# Observability

## Purpose

Describe what the owner can see while the agent runs. Observation is
read-only and never perturbs the loop: looking at the agent costs it nothing.

## Surfaces

| Surface | Content |
| --- | --- |
| `lkjagent status` | Daemon state, queue depth, open task title, turns taken, context usage, last compaction |
| `lkjagent log` | Transcript events in order: messages, actions, observations, notices, compactions |
| `lkjagent memory` | Full-text search over distilled memory entries |
| `lkjagent skills` | Skill library index with refinement timestamps |
| sqlite3 on the store | Everything, for forensics; schema in [../architecture/memory/store.md](../architecture/memory/store.md) |

## Transcript as Truth

The transcript is the complete account of agent behavior: every action the
model took, every observation it saw, every notice the harness injected, and
every compaction. If something is not in the transcript, it did not happen.
Event kinds are defined in
[../architecture/memory/transcripts.md](../architecture/memory/transcripts.md).

`lkjagent log` renders events compactly: one line per event with kind, turn,
and a bounded preview; `--full` prints whole payloads.

## Context Usage

`lkjagent status` reports the live token ledger: prefix size, log size,
remaining headroom, and the compaction threshold from
[../architecture/context/budgets.md](../architecture/context/budgets.md).
This makes context pollution visible long before it hurts.

## What Does Not Exist

No metrics endpoint, no dashboard, no log shipping. The store is local and
the CLI reads it. Anything fancier is a skill the agent can build on demand.

## Status

design-only.
