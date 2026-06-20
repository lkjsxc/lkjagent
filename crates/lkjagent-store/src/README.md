# lkjagent-store Source

## Purpose

This directory holds schema setup and typed SQLite store APIs.

## Table of Contents

- [error.rs](error.rs): store error type.
- [events.rs](events.rs): append-only transcript events.
- [graph/](graph/README.md): graph table helper modules.
- [graph.rs](graph.rs): graph case, event, evidence, and memory-link APIs.
- [lib.rs](lib.rs): library root.
- [memory.rs](memory.rs): memory write, edit, search, and digest APIs.
- [memory/](memory/README.md): memory row mapping helpers.
- [queue.rs](queue.rs): queue delivery and mutation APIs.
- [queue/](queue/README.md): queue row mapping helpers.
- [schema.rs](schema.rs): current SQLite schema setup.
- [schema_graph.rs](schema_graph.rs): graph-specific SQLite schema setup.
- [state.rs](state.rs): key-value state and daemon lock decisions.
- [token_usage.rs](token_usage.rs): normalized token usage event ledger.
