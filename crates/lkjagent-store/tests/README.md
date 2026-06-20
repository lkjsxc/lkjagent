# lkjagent-store Tests

## Purpose

This directory holds in-memory SQLite integration tests for the store crate.

## Table of Contents

- [events.rs](events.rs): append/read event ordering fixture.
- [graph.rs](graph.rs): graph case, evidence, and memory-link fixtures.
- [memory.rs](memory.rs): FTS ranking, digest, update, and delete fixtures.
- [queue.rs](queue.rs): delivery and mutation fixtures.
- [state.rs](state.rs): daemon lock fixture.
- [token_usage.rs](token_usage.rs): token usage ledger fixtures.
- [support/](support/README.md): shared in-memory setup helper.
