# Store

## Purpose

The schema and access rules of the SQLite store: one file, queue tables,
graph tables, transcript tables, memory tables, and the transaction
discipline that lets the daemon and the thin CLI share it without IPC.

## Location and Access

The store is one SQLite file at /data/lkjagent.sqlite3 inside the
container, backed by LKJAGENT_DATA_DIR per
[../sandbox/workspace.md](../sandbox/workspace.md).
It runs in WAL mode so the daemon and the thin CLI share the file safely:
the CLI and tools call lkjagent-store directly, and no socket or IPC
protocol exists. Queue writes use store APIs only; SQL strings for queue
mutation do not appear outside this crate. sqlite3 on the file is the
read-only forensics surface per
[../../product/observability.md](../../product/observability.md).

## Tables

### queue

The persistent owner-message queue, drained by the loop at turn boundaries.

| Column | Type | Meaning |
| --- | --- | --- |
| id | INTEGER PRIMARY KEY | delivery order |
| created_at | TEXT | enqueue time |
| updated_at | TEXT | last mutation time |
| source_queue_id | INTEGER | source row for redelivery; null for original enqueue |
| content | TEXT | the owner message, verbatim |
| status | TEXT | pending, delivered, or deleted |
| delivered_turn | INTEGER | the turn that received the row; null while pending |

### events

The transcript: append-only, never updated, never deleted.

| Column | Type | Meaning |
| --- | --- | --- |
| id | INTEGER PRIMARY KEY | total order of the transcript |
| turn | INTEGER | owning turn; null for out-of-turn queue mutation events |
| kind | TEXT | owner, action, observation, notice, queue_mutation, compaction, or error |
| content | TEXT | the event payload |
| tokens | INTEGER | token count of the payload as windowed |
| created_at | TEXT | write time |

Event semantics are owned by [transcripts.md](transcripts.md).

### memory

Distilled durable knowledge, written per [distillation.md](distillation.md).

| Column | Type | Meaning |
| --- | --- | --- |
| id | INTEGER PRIMARY KEY | row id |
| kind | TEXT | lesson, fact, task-summary, or incident |
| title | TEXT | searchable noun phrase |
| tags | TEXT | files, tools, and subsystems touched |
| content | TEXT | the entry body |
| tokens | INTEGER | budget cost used by digest selection |
| created_at | TEXT | first write |
| updated_at | TEXT | last edit; the recency key in [retrieval.md](retrieval.md) |

memory_fts is an FTS5 mirror over title, tags, and content. It is kept in
sync with memory inside the same transaction as every memory write.
`memory.find` normalizes raw search text before MATCH so punctuation such as
tool names, brackets, and hyphens cannot create an FTS syntax loop.

### graph_cases

One row per active or closed task case.

| Column | Type | Meaning |
| --- | --- | --- |
| id | INTEGER PRIMARY KEY | case id |
| status | TEXT | active, waiting, closed, or paused |
| objective | TEXT | owner objective or maintenance directive |
| raw_owner_text | TEXT | original owner text used for reconstruction |
| objective revision | INTEGER | objective revision counter |
| task_family | TEXT | classified graph task family |
| subroute | TEXT | narrower task route |
| route_reason | TEXT | classifier reason for the route |
| phase | TEXT | active task phase |
| node_id | TEXT | active graph node |
| plan | TEXT | structured plan and current next actions |
| evidence_requirements | TEXT | bounded line list of missing proof |
| selected_packages | TEXT | bounded line list of package ids |
| pending_checks | TEXT | bounded line list of checks still due |
| next_action_class | TEXT | preferred next action class |
| context_pressure | TEXT | green, yellow, orange, red, or black-invalid |
| created_at | TEXT | first write |
| updated_at | TEXT | last graph state write |
| closed_at | TEXT | close time, null while open |

### graph_events

Transition events, selected packages, phase changes, and recovery routing.

| Column | Type | Meaning |
| --- | --- | --- |
| id | INTEGER PRIMARY KEY | graph event id |
| case_id | INTEGER | owning graph case |
| kind | TEXT | transition, context, recovery, completion, or maintenance |
| node_id | TEXT | graph node related to the event |
| content | TEXT | compact event payload |
| created_at | TEXT | write time |

### graph_evidence

Observed files, command results, verification outputs, read facts, and
completion proof.

| Column | Type | Meaning |
| --- | --- | --- |
| id | INTEGER PRIMARY KEY | evidence id |
| case_id | INTEGER | owning graph case |
| requirement | TEXT | evidence requirement or inferred category |
| kind | TEXT | owner, action, observation, verification, file, memory, or note |
| summary | TEXT | bounded evidence summary |
| path | TEXT | optional workspace path |
| event_id | INTEGER | transcript event link when present |
| created_at | TEXT | write time |

`graph_plan_steps`, `graph_context_bindings`, `graph_artifacts`,
`graph_document_state`, `graph_faults`, `graph_recovery_state`,
`graph_compaction_snapshots`, and `graph_transitions` store the normalized
ledger around the header. Context bindings include compression level so
restart can rebuild the same pressure-aware graph card.

`graph_memory_links` links memory rows to graph cases and nodes. Task-summary
memory rows are linked to the active or just-closed case node when saved.
`graph_node_stats` stores deterministic counters used for ranking package
choices.

### state

Key-value runtime state: key TEXT PRIMARY KEY, value TEXT.

| Key | Holds |
| --- | --- |
| daemon lock | holder id, start time, and heartbeat per [../runtime/daemon-process.md](../runtime/daemon-process.md) |
| daemon state | idle, working, waiting, or error |
| daemon question | outstanding agent.ask text, if any |
| daemon error | latest endpoint or loop error, if any |
| maintenance stamps | per-directive explicit-maintenance stamps, [../runtime/self-maintenance.md](../runtime/self-maintenance.md) |
| counters | turn counter and similar running totals |

## Transactions

Every write is one transaction. Queue delivery marks the row delivered and
writes the owner event in the same transaction, so a message is never both
delivered and missing from the transcript. A SIGKILL loses at most one
in-flight turn; the queue and the transcript stay consistent.

Queue mutation APIs, including CLI send and the queue tools, enqueue, edit,
delete, and redeliver rows in the same transaction as their queue_mutation
event. That event records operation, reason, target id, source_queue_id
when present, and before and after content where applicable. CLI send uses
the fixed reason `owner-send`.

## Mutability

- events rows are append-only: no update, no delete, ever. The transcript
  is the complete truth of agent behavior.
- queue rows mutate only through lkjagent-store queue APIs. Pending rows
  can be edited or tombstoned; delivered rows keep their delivered content
  and turn. Redelivery inserts a new source-linked pending row instead of
  rewriting delivered history.
- memory rows may be updated and deleted, but only through memory.save,
  memory.prune, and the prune-memory directive per
  [distillation.md](distillation.md).
- memory.save is idempotent by default; equivalent rows return or update the
  existing row rather than inserting a duplicate.

## Deliberately Not Stored

- Source graph definitions: the store holds runtime cases, events, evidence,
  and ranking stats, not the source graph itself.
- Config: data/lkjagent.json on disk, never mirrored into tables.
- The endpoint API key: it arrives by environment variable per
  [../sandbox/container.md](../sandbox/container.md) and is never written
  to the store.

## Status

implemented.
