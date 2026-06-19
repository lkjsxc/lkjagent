# Daemon Helpers

## Purpose

This directory holds adapter helpers for the foreground daemon.

## Table of Contents

- [compaction.rs](compaction.rs): distillation cycle and rebuild handoff.
- [compaction_gate.rs](compaction_gate.rs): compaction-only action gate.
- [compaction_support.rs](compaction_support.rs): compaction prompt helpers.
- [context_budget.rs](context_budget.rs): turn-boundary pressure checks.
- [count_scaffold.rs](count_scaffold.rs): counted document auto scaffold.
- [effects.rs](effects.rs): step effect persistence and tool dispatch.
- [endpoint.rs](endpoint.rs): endpoint completion and oversize handling.
- [idle.rs](idle.rs): automatic idle maintenance cycle opening.
- [maintenance_wait.rs](maintenance_wait.rs): maintenance ask auto-close guard.
- [persisted.rs](persisted.rs): store-backed guard restoration and owner previews.
- [pressure.rs](pressure.rs): pressure state names.
- [record.rs](record.rs): compaction transcript recording.
- [runner.rs](runner.rs): resident poll loop and effect interpretation.
- [scaffold.rs](scaffold.rs): graph evidence and scaffold recording helpers.
- [skills.rs](skills.rs): guarded task skill auto-loading and scaffolding.
- [status.rs](status.rs): daemon state fields written to the store.
- [startup.rs](startup.rs): seed copying and prefix input loading.
