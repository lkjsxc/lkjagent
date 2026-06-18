# Store

## Purpose

The schema and access rules of the SQLite store: one file, four tables, one
full-text mirror, and the transaction discipline that lets the daemon and
the thin CLI share it without IPC.

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

### state

Key-value runtime state: key TEXT PRIMARY KEY, value TEXT.

| Key | Holds |
| --- | --- |
| daemon lock | holder pid, start time, and heartbeat per [../runtime/daemon-process.md](../runtime/daemon-process.md) |
| daemon state | idle, working, waiting, or error |
| daemon question | outstanding agent.ask text, if any |
| daemon error | latest endpoint or loop error, if any |
| open task | the current task label, or none |
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
- memory rows may be updated and deleted, but only through memory.save and
  the prune-memory directive per [distillation.md](distillation.md).

## Deliberately Not Stored

- Skills: markdown files in the skill library directory at /data/skills.
  The store holds only their index stamps.
- Config: data/lkjagent.json on disk, never mirrored into tables.
- The endpoint API key: it arrives by environment variable per
  [../sandbox/container.md](../sandbox/container.md) and is never written
  to the store.

## Status

implemented.
