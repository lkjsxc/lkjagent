# Rows

## Purpose

Define the durable memory row contract.

## Table

Memory uses one content table plus an FTS mirror owned by the store schema.

| Field | Meaning |
| --- | --- |
| `id` | stable row id |
| `created_at` | UTC creation timestamp |
| `topic` | short label used for ranking and deduplication |
| `content` | distilled text capped by `memory.distill.words=120` |
| `task_id` | task provenance when available |

Exact duplicate topic and content pairs are ignored. Memory pruning is an owner
CLI action, not idle behavior.

## Prompt Admission

A task brief may carry at most `context.memory.fact-tokens=100` of memory facts
and must label their provenance. Write steps do not receive unrelated memory by
default.
