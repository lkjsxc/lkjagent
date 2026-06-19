# SQLite Store

## Purpose

Implement lkjagent-store: schema setup, queue delivery and mutation APIs,
append-only events, editable memory with FTS retrieval, the state table,
and the daemon lock row.

## Status

done

## Depends On

[bootstrap-workspace.md](bootstrap-workspace.md); types from
[protocol-parser.md](protocol-parser.md) for event kinds.

## Files To Read

- [../../architecture/memory/store.md](../../architecture/memory/store.md)
- [../../architecture/memory/transcripts.md](../../architecture/memory/transcripts.md)
- [../../architecture/memory/retrieval.md](../../architecture/memory/retrieval.md)
- [../../architecture/memory/distillation.md](../../architecture/memory/distillation.md)

## Files To Touch

- crates/lkjagent-store/src/: schema.rs, queue.rs, events.rs, memory.rs
  (rows plus FTS queries plus digest selection), state.rs (lock row,
  stamps), error.rs.
- crates/lkjagent-store/tests/: in-memory SQLite tables for delivery,
  ordering, ranking, digest budget.

## Focused Gate

```sh
cargo test -p lkjagent-store
cargo clippy -p lkjagent-store -- -D warnings
```

## Acceptance

- Queue delivery marks the row and writes the owner event in one
  transaction; the exactly-once test passes under interleaved writers.
- Queue rows include updated_at, source_queue_id, and status values pending,
  delivered, and deleted.
- Queue enqueue, edit, delete, and redeliver APIs write a queue_mutation
  event in the same transaction, including operation, reason, target id,
  source link, and before and after content where applicable.
- queue.delete is a tombstone; queue.redeliver creates a new pending row
  linked by source_queue_id; delivered owner events are never rewritten.
- The events API exposes append and read only; no update or delete
  functions exist.
- memory.find ranking honors bm25 with kind weights and recency tiebreak
  per the retrieval contract's table.
- Digest selection fills the 2,048-token budget, task summary first.
- Lock row take, refuse, and stale-reclaim behaviors pass their tests.
- Blocker row 5 done; memory area status moves in the ledger.

## Must Not

- Do not let SQL strings appear outside this crate.
- Do not write migration logic; setup creates the current schema, reset
  semantics apply, and the handoff says so.
- Do not store source graph definitions or secrets.
