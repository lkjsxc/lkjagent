# lkjagent-store Source

## Purpose

This directory holds schema setup and typed SQLite store APIs.

## Table of Contents

- [artifact_cursor.rs](artifact_cursor.rs): semantic artifact batch cursor APIs.
- [artifact_ledger.rs](artifact_ledger.rs): semantic artifact ledger APIs.
- [error.rs](error.rs): store error type.
- [events.rs](events.rs): append-only transcript events.
- [graph/](graph/README.md): graph table helper modules.
- [graph.rs](graph.rs): graph case, event, evidence, and memory-link APIs.
- [lib.rs](lib.rs): library root.
- [memory.rs](memory.rs): memory write, edit, search, and digest APIs.
- [memory/](memory/README.md): memory row mapping helpers.
- [personal.rs](personal.rs): diary, schedule, and TODO store API exports.
- [personal/](personal/README.md): personal record models, validation, reads, and writes.
- [queue.rs](queue.rs): queue delivery and mutation APIs.
- [queue/](queue/README.md): queue row mapping helpers.
- [runtime_authority.rs](runtime_authority.rs): authority history API exports.
- [runtime_authority/](runtime_authority/README.md): authority rows, reads, writes, and codecs.
- [schema.rs](schema.rs): current SQLite schema setup.
- [schema_artifacts.rs](schema_artifacts.rs): semantic artifact ledger SQLite schema setup.
- [schema_authority.rs](schema_authority.rs): runtime authority SQLite schema setup.
- [schema_graph.rs](schema_graph.rs): graph-specific SQLite schema setup.
- [schema_personal.rs](schema_personal.rs): personal record SQLite schema setup.
- [state.rs](state.rs): key-value state and daemon lock decisions.
- [token_usage.rs](token_usage.rs): normalized token usage event ledger.
- [provider_exchange.rs](provider_exchange.rs): provider exchange source module.
- [schema_provider_exchange.rs](schema_provider_exchange.rs): schema provider exchange source module.
- [token_usage_aggregate.rs](token_usage_aggregate.rs): token usage aggregate source module.
