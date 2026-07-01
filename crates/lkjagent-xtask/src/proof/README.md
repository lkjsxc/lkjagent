# Proof Collector Source

## Purpose

This directory holds the `proof collect` implementation: bounded store
queries, file indexes, word counts, Markdown rendering, and focused tests.

## Table of Contents

- [collect.rs](collect.rs): proof bundle orchestration and file writes.
- [db.rs](db.rs): bounded SQLite metadata queries.
- [db-support.rs](db_support.rs): database warning and truncation helpers.
- [files.rs](files.rs): workspace, model-log, and word-count scans.
- [model.rs](model.rs): proof bundle data records.
- [render.rs](render.rs): Markdown and text rendering helpers.
- [render-rows.rs](render_rows.rs): tabular proof row conversion helpers.
- [tests.rs](tests.rs): empty and seeded proof bundle tests.
