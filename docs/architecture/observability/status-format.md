# Status Format

## Purpose

This file defines plain text `lkjagent status` output.

## Contract

- Include runtime, queue, task, authority, artifact, context, token, model-log,
  next-action, and diagnostic sections.
- Use stable prefixed key-value lines for machine and human scanning, such as
  `runtime.daemon_state`, `queue.pending`, `task.active_case`,
  `artifact.root`, `tokens.usage`, and `next.action`.
- Render unknown token values as `unknown`.
- Keep counts compact with the same formatter and deck as the console.

## Implementation Hooks

- Source: `crates/lkjagent-cli/src/accounting.rs`
- Source: `crates/lkjagent-cli/src/status.rs`
- Tests: `crates/lkjagent-cli/tests/status.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- Status reports context size without total window or percentage.
- Token fields are absent from a running task.
- model log path points to a stale or competing current file.

## Status

implemented for the current prefixed status deck shared with the console.
