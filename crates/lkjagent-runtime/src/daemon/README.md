# Daemon Helpers

## Purpose

This directory holds adapter helpers for the foreground daemon.

## Table of Contents

- [compaction.rs](compaction.rs): runtime-owned compaction and rebuild handoff.
- [compaction_support.rs](compaction_support.rs): compaction summary helpers.
- [authority.rs](authority.rs): store-backed turn authority snapshots.
- [authority_admission.rs](authority_admission.rs): normalized dispatch admission writes.
- [authority_ledger.rs](authority_ledger.rs): normalized authority event and decision writes.
- [authority_store.rs](authority_store.rs): flat status-key authority snapshot writes.
- [context_budget.rs](context_budget.rs): turn-boundary pressure checks.
- [count_scaffold.rs](count_scaffold.rs): counted document auto scaffold.
- [count_scaffold_gate.rs](count_scaffold_gate.rs): graph gate check for counted scaffold closure.
- [effects.rs](effects.rs): step effect persistence.
- [effects_graph.rs](effects_graph.rs): graph effect persistence helpers.
- [execute_pending.rs](execute_pending.rs): tool dispatch after runtime gates.
- [endpoint.rs](endpoint.rs): endpoint completion and oversize handling.
- [idle.rs](idle.rs): automatic idle maintenance cycle opening.
- [maintenance_wait.rs](maintenance_wait.rs): maintenance ask auto-close guard.
- [owner_delivery.rs](owner_delivery.rs): queue delivery and owner step opening.
- [persisted.rs](persisted.rs): store-backed guard restoration and owner previews.
- [pressure.rs](pressure.rs): pressure state names.
- [record.rs](record.rs): compaction transcript recording.
- [runner.rs](runner.rs): resident poll loop and effect interpretation.
- [scaffold.rs](scaffold.rs): graph evidence and scaffold recording helpers.
- [scaffold_evidence.rs](scaffold_evidence.rs): scaffold graph evidence persistence.
- [graph_policy.rs](graph_policy.rs): graph dispatch policy and ledger-aware completion helpers.
- [graph_sync.rs](graph_sync.rs): graph policy synchronization for dispatch.
- [status.rs](status.rs): daemon state fields written to the store.
- [startup.rs](startup.rs): seed copying and prefix input loading.
