# Daemon Effects Helpers

## Purpose

This directory owns daemon effect persistence, pending dispatch execution,
graph effect writes, and compaction transcript records.

## Table of Contents

- [effects.rs](effects.rs): step effect persistence.
- [effects_graph.rs](effects_graph.rs): graph effect persistence helpers.
- [execute_pending.rs](execute_pending.rs): tool dispatch after runtime gates.
- [pending_observation.rs](pending_observation.rs): authority effect and observation writes.
- [pending_staleness.rs](pending_staleness.rs): stale pending action checks.
- [pending_staleness_tests.rs](pending_staleness_tests.rs): pending staleness tests.
- [record.rs](record.rs): compaction transcript recording.
- [effects_graph_evidence.rs](effects_graph_evidence.rs): effects graph evidence source module.
