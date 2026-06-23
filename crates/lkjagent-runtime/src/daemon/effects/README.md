# Daemon Effects Helpers

## Purpose

This directory owns daemon effect persistence, pending dispatch execution,
graph effect writes, and compaction transcript records.

## Table of Contents

- [effects.rs](effects.rs): step effect persistence.
- [effects_graph.rs](effects_graph.rs): graph effect persistence helpers.
- [execute_pending.rs](execute_pending.rs): tool dispatch after runtime gates.
- [record.rs](record.rs): compaction transcript recording.
