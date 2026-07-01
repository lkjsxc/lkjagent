# Daemon Authority Helpers

## Purpose

This directory owns daemon adapters for authority snapshots, decisions,
admission rows, and graph-policy synchronization.

## Table of Contents

- [authority.rs](authority.rs): store-backed turn authority snapshots.
- [authority_admission.rs](authority_admission.rs): normalized dispatch admission writes.
- [graph_policy.rs](graph_policy.rs): graph dispatch policy and ledger-aware completion helpers.
- [graph_snapshot.rs](graph_snapshot.rs): graph fields used by runtime authority snapshots.
- [graph_sync.rs](graph_sync.rs): graph policy synchronization for dispatch.
- [kernel_turn.rs](kernel_turn.rs): daemon adapter for persisted kernel turn decisions.
- [kernel_turn_input.rs](kernel_turn_input.rs): snapshot and event input mapping for kernel turns.
- [kernel_turn_persist.rs](kernel_turn_persist.rs): status-key projection from kernel turn records.
- [kernel_turn_policy.rs](kernel_turn_policy.rs): dispatch policy projection from kernel decisions.
- [kernel_turn_artifact.rs](kernel_turn_artifact.rs): kernel turn artifact source module.
