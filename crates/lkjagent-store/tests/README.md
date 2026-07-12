# lkjagent-store Tests

## Purpose

This directory holds in-memory SQLite integration tests for the store crate.

## Table of Contents

- [artifacts.rs](artifacts.rs): artifact rows and fingerprint metadata.
- [durable-boundaries.rs](durable_boundaries.rs): native setup, policy, byte, and compatibility boundaries.
- [native-constraints.rs](native_constraints.rs): native relational cardinality and identity constraints.
- [native-reopen.rs](native_reopen.rs): restart and altered-schema rejection.
- [native-transactions.rs](native_transactions.rs): intake, compilation, provider, and effect atomicity.
- [native-settlement.rs](native_settlement.rs): observation settlement and guarded close atomicity.
- [plan-store.rs](plan_store.rs): plan store schema, queue, turn, and inspection fixtures.
- [state-ledger.rs](state_ledger.rs): state cells, decisions, and context rows.
- [workspace-rows.rs](workspace_rows.rs): manifest, path alias, and rebalance audit rows.
