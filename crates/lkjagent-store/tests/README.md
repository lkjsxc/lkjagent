# lkjagent-store Tests

## Purpose

This directory holds in-memory SQLite integration tests for the store crate.

## Table of Contents

- [artifact_cursor.rs](artifact_cursor.rs): artifact batch cursor fixtures.
- [artifact_ledger.rs](artifact_ledger.rs): semantic artifact ledger and weak-path fixtures.
- [compaction_snapshot.rs](compaction_snapshot.rs): compaction snapshot reopen fixtures.
- [events.rs](events.rs): append/read event ordering fixture.
- [graph.rs](graph.rs): graph case, evidence, and memory-link fixtures.
- [memory.rs](memory.rs): FTS ranking, digest, update, and delete fixtures.
- [memory_prune.rs](memory_prune.rs): exact and semantic prune fixtures.
- [queue.rs](queue.rs): delivery and mutation fixtures.
- [runtime_authority.rs](runtime_authority.rs): authority event, decision, and admission fixtures.
- [state.rs](state.rs): daemon lock fixture.
- [token_usage.rs](token_usage.rs): token usage ledger fixtures.
- [support/](support/README.md): shared in-memory setup helper.
