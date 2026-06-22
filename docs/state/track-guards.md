# Track Guards

## Purpose

Track guards own the thresholds that block tools or alter next-action choice.
The dispatcher must check hard state first, then guard tracks.

## Guard Table

| Track | Threshold | Block |
| --- | --- | --- |
| mock content risk | 0.70 | `agent.done` |
| model-specific naming | 0.60 | `memory.save`, `agent.done` |
| structure connectivity | 0.60 | `agent.done` |
| parse recovery | 0.80 | large `fs.batch_write` and `artifact.apply` |
| artifact drift | 0.75 | `artifact.next` and `artifact.apply` |
| repeated action risk | 0.60 | identical action signature |
| context pressure | 0.85 | mutating tools before snapshot check |
| queue interruption | 0.70 | mutating tools before classification |

## Tool Bias

Active guards also suggest smaller tools: `doc.audit`, `fs.tree`, `fs.read`,
`graph.state`, `queue.list`, and the narrow repair action named by the failure.

## Status

implemented
