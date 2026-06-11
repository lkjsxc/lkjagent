# Memory Operations

## Purpose

The contracts for memory.save and memory.find: how the model writes
durable knowledge into the store and queries it back. The row schema is
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

The observation is the new memory row id.

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
| query | required, full-text query |
| limit | optional, default 5 |

The observation lists ranked entries, each as id, kind, title, and a
bounded snippet. Ranking is owned by
[../memory/retrieval.md](../memory/retrieval.md). Searching memory before
re-reading files is the cheap path under
[../context/hygiene.md](../context/hygiene.md).

## Status

design-only.
