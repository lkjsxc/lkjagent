# Queue Operations

## Purpose

The contracts for queue.list and the four queue mutation tools. These tools
let the agent inspect and shape future work without rewriting delivered
history. Canonical parameter table: [registry.md](registry.md).

## queue.list

| Parameter | Rule |
| --- | --- |
| status | optional, one of all, pending, delivered, deleted; default all |
| limit | optional, default 20 |

The observation lists rows by id with status, source_queue_id, created_at,
updated_at, and a bounded content preview. queue.list is read-only and
writes no transcript event.

## Mutation Rules

The mutating queue tools are queue.enqueue, queue.edit, queue.delete, and
queue.redeliver. They call lkjagent-store APIs only; SQL strings do not
appear in tool adapters, the runtime, or the CLI.

Every queue mutation writes a queue_mutation transcript event in the same
transaction as the queue row change. The event payload records operation,
reason, target id, source_queue_id when present, and before and after
content where applicable.

## queue.enqueue

| Parameter | Rule |
| --- | --- |
| content | required |
| reason | required |

Creates a pending row with source_queue_id null. The new row is eligible for
future delivery at the next turn boundary after older pending rows.

## queue.edit

| Parameter | Rule |
| --- | --- |
| id | required |
| content | required |
| reason | required |

Replaces the content of a pending row and updates updated_at. Delivered and
deleted rows are not editable; their history stays in the transcript.

## queue.delete

| Parameter | Rule |
| --- | --- |
| id | required |
| reason | required |

Marks a pending row deleted. This is a tombstone: the row remains in the
store and transcript audit trail but is skipped by future delivery.

## queue.redeliver

| Parameter | Rule |
| --- | --- |
| id | required |
| reason | required |
| content | optional |

Creates a new pending row with source_queue_id set to the selected row. If
content is omitted, the new row copies the source content; otherwise it uses
the supplied content. The source row and any delivered owner event are never
rewritten.

## Status

implemented.
