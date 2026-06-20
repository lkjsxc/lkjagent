# Memory Operations

## Purpose

The contracts for memory.save, memory.find, and memory.prune: how the model
writes durable knowledge into the store, queries it back, and deletes exact
duplicates during maintenance. The row schema is
owned by [../memory/store.md](../memory/store.md), ranking by
[../memory/retrieval.md](../memory/retrieval.md), and the judgment of when
to save by [../memory/distillation.md](../memory/distillation.md).
Canonical parameter table: [registry.md](registry.md).

## memory.save

| Parameter | Rule |
| --- | --- |
| kind | required, one of lesson, fact, task-summary, incident |
| title | required, one line |
| tags | optional, space-separated |
| content | required |

The observation is the memory row id. Equivalent rows return the existing id
with `duplicate=skipped` instead of inserting another row.

## Kinds

| Kind | Meaning |
| --- | --- |
| lesson | a transferable rule learned from an outcome, stated so it changes future behavior |
| fact | a stable statement about the workspace, the owner, or the environment |
| task-summary | the distilled result of one task: what was asked, done, and verified |
| incident | a failure record: what broke, the cause, and the signal that detects recurrence |

## memory.find

| Parameter | Rule |
| --- | --- |
| query | required, normalized before FTS lookup |
| limit | optional, default 5 |

The observation lists ranked entries, each as id, kind, title, and a
bounded snippet. Ranking is owned by
[../memory/retrieval.md](../memory/retrieval.md). Searching memory before
re-reading files is the cheap path under
[../context/hygiene.md](../context/hygiene.md).

Punctuation that FTS5 treats as syntax is split into searchable tokens. When
normalization changes a query, the observation includes `query_normalized`.

## memory.prune

`memory.prune` takes no parameters. It deletes exact duplicate memory rows
using the store identity key and reports kept duplicate groups plus deleted
rows. It is admitted only in maintenance mode.

## Status

implemented for accepted kinds, idempotent duplicate skips, and punctuation
safe FTS queries. Exact duplicate pruning is implemented; semantic merge and
rewrite pruning remain open.
