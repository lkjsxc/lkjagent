# Daemon Context Helpers

## Purpose

This directory owns context budget checks, runtime-owned compaction, persisted
guard restoration, and pressure names.

## Table of Contents

- [context_budget.rs](context_budget.rs): turn-boundary pressure checks.
- [compaction.rs](compaction.rs): runtime-owned compaction and rebuild handoff.
- [compaction_support.rs](compaction_support.rs): compaction summary helpers.
- [persisted.rs](persisted.rs): store-backed guard restoration and owner previews.
- [pressure.rs](pressure.rs): pressure state names.
