# Daemon Authority Helpers

## Purpose

This directory owns daemon adapters for authority snapshots, decisions,
admission rows, and graph-policy synchronization.

## Table of Contents

- [authority.rs](authority.rs): store-backed turn authority snapshots.
- [authority_admission.rs](authority_admission.rs): normalized dispatch admission writes.
- [authority_ledger.rs](authority_ledger.rs): normalized authority snapshot, event, decision, and transition writes.
- [authority_ledger_support.rs](authority_ledger_support.rs): authority ledger helper fields and fingerprints.
- [authority_store.rs](authority_store.rs): flat status-key authority snapshot writes.
- [graph_policy.rs](graph_policy.rs): graph dispatch policy and ledger-aware completion helpers.
- [graph_sync.rs](graph_sync.rs): graph policy synchronization for dispatch.
