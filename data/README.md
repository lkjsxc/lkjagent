# Data

## Purpose

This directory holds tracked diagnostic evidence plus ignored local runtime
state for lkjagent.

## Contents

- [workspace/](workspace/README.md): tracked workspace evidence and the
  runtime workspace brief.
- [logs/](logs/): tracked model-run evidence used by the current-state ledger.
- `lkjagent.sqlite3`, `lkjagent.sqlite3-shm`, `lkjagent.sqlite3-wal`, and
  `lkjagent.json`: local runtime state ignored by Git.

## Contract

`data/workspace` and `data/logs` are evidence surfaces, not completed product
artifacts. A generated output under this tree is complete only when its own
audit evidence says the objective-specific profile passed.

The runtime layout is defined by
[../docs/architecture/sandbox/workspace.md](../docs/architecture/sandbox/workspace.md).
The honest project ledger is
[../docs/current-state.md](../docs/current-state.md).
