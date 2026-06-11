# Store

## Purpose

The schema and access rules of the SQLite store: one file, four tables, one
full-text mirror, and the transaction discipline that lets the daemon and
the thin CLI share it without IPC.

## Location and Access

The store is one SQLite file at data/lkjagent.sqlite3, on the /data volume
inside the container per [../sandbox/workspace.md](../sandbox/workspace.md).
It runs in WAL mode so the daemon and the thin CLI share the file safely:
the CLI writes queue rows and reads transcript rows directly, and no socket
or IPC protocol exists. sqlite3 on the file is the forensics surface per
[../../product/observability.md](../../product/observability.md).

## Tables

### queue

The persistent owner-message queue, drained by the loop at turn boundaries.

| Column | Type | Meaning |
| --- | --- | --- |
| id | INTEGER PRIMARY KEY | delivery order |
| created_at | TEXT | enqueue time |
| content | TEXT | the owner message, verbatim |
| status | TEXT | pending or delivered |
| delivered_turn | INTEGER | the turn that received the row; null while pending |

### events

The transcript: append-only, never updated, never deleted.

| Column | Type | Meaning |
| --- | --- | --- |
| id | INTEGER PRIMARY KEY | total order of the transcript |
| turn | INTEGER | the turn the event belongs to |
| kind | TEXT | owner, action, observation, notice, compaction, or error |
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
| created_at | TEXT | first write |
| updated_at | TEXT | last edit; the recency key in [retrieval.md](retrieval.md) |

memory_fts is an FTS5 mirror over title, tags, and content. It is kept in
sync with memory inside the same transaction as every memory write.

### state

Key-value runtime state: key TEXT PRIMARY KEY, value TEXT.

| Key | Holds |
| --- | --- |
| daemon lock | holder pid and start time per [../runtime/daemon-process.md](../runtime/daemon-process.md) |
| open task | the current task, if any |
| maintenance stamps | per-directive last-run stamps, [../runtime/self-maintenance.md](../runtime/self-maintenance.md) |
| counters | turn counter and similar running totals |

## Transactions

Every write is one transaction. Queue delivery marks the row delivered and
writes the owner event in the same transaction, so a message is never both
delivered and missing from the transcript. A SIGKILL loses at most one
in-flight turn; the queue and the transcript stay consistent.

## Mutability

- events rows are append-only: no update, no delete, ever. The transcript
  is the complete truth of agent behavior.
- memory rows may be updated and deleted, but only through memory.save and
  the prune-memory directive per [distillation.md](distillation.md).

## Deliberately Not Stored

- Skills: markdown files in the skill library directory at /data/skills.
  The store holds only their index stamps.
- Config: data/lkjagent.toml on disk, never mirrored into tables.
- The endpoint API key: it arrives by environment variable per
  [../sandbox/container.md](../sandbox/container.md) and is never written
  to the store.

## Status

design-only.
