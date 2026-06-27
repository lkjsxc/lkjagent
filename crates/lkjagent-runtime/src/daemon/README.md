# Daemon Helpers

## Purpose

This directory holds adapter helpers for the foreground daemon, grouped by
effect boundary and status surface.

## Table of Contents

- [authority/](authority/README.md): authority snapshots, admissions, ledgers, and graph policy sync.
- [loop/](loop/README.md): resident loop, startup, endpoint, queue, and idle helpers.
- [context/](context/README.md): context budget, compaction, persisted guard, and pressure helpers.
- [effects/](effects/README.md): effect persistence, pending dispatch, graph effects, and transcript records.
- [artifacts/](artifacts/README.md): scaffold, counted scaffold, and artifact evidence helpers.
- [shutdown.rs](shutdown.rs): signal-to-shutdown decision helpers.
- [status/](status/README.md): status fields and task summary rendering.
