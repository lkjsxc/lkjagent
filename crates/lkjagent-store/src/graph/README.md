# lkjagent-store Graph Helpers

## Purpose

This directory holds graph-table helper modules that are split out of the
public graph store API when a table needs focused ownership.

## Table of Contents

- [artifacts.rs](artifacts.rs): graph artifact record helpers.
- [cases.rs](cases.rs): graph case header create, read, and update helpers.
- [context.rs](context.rs): selected context package binding helpers.
- [documents.rs](documents.rs): document topology state helpers.
- [faults.rs](faults.rs): fault and recovery ladder persistence helpers.
- [links.rs](links.rs): graph case to memory row link helpers.
- [notes.rs](notes.rs): non-evidence note persistence helpers.
- [plan.rs](plan.rs): structured plan step persistence helpers.
- [snapshots.rs](snapshots.rs): compaction snapshot persistence helpers.
- [state_tracks.rs](state_tracks.rs): neutral multi-state progress persistence.
- [transitions.rs](transitions.rs): graph transition event helpers.
