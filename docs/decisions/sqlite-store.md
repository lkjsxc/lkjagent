# SQLite Store

## Purpose

Fix where queue, transcripts, memory, and runtime state persist.

## Decision

One SQLite database file holds the user message queue, the transcript event
log, distilled memory with a full-text index, and runtime state. The schema
is owned by [../architecture/memory/store.md](../architecture/memory/store.md).
Skills are the exception: they live as source-owned markdown files per
[../decisions/unified-skills.md](unified-skills.md), and the store only
indexes usage state.

WAL mode lets the daemon and the thin CLI share the file safely; the CLI
writes queue rows and reads transcript rows without any IPC protocol.

## Consequences

- One file to back up, inspect, and reason about; sqlite3 is the debugger.
- FTS gives memory retrieval without an embedding stack; relevance is
  lexical, which suits exact-name recall of files, tools, and decisions.
- The store is the CLI-daemon interface; no socket server exists, so the
  attack and failure surface stays minimal.
- Transcript rows are append-only, matching the context discipline in
  [append-only-context.md](append-only-context.md).

## Rejected Directions

- Plain JSONL and markdown files for everything: directly readable, but
  queue claiming, cross-process safety, and search all require inventing
  fragile machinery SQLite already provides.
- An embedding-based vector store: heavy, opaque, and unnecessary at this
  scale; FTS plus good distillation wins on a 16 GB box.
- A client-server database: absurd for one owner in one container.
