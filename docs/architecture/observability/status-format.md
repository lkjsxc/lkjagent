# Status Format

## Purpose

This file defines plain text `lkjagent status` output.

## Contract

- Include daemon state, queue depth, active case, top state tracks, context
  fraction, token usage, last fault, last action, owner question, and GPT log path.
- Use stable key-value lines for machine and human scanning.
- Render unknown token values as `unknown`.
- Keep counts compact with the same formatter as the console.

## Implementation Hooks

- Source: `crates/lkjagent-cli/src/accounting.rs`
- Source: `crates/lkjagent-cli/src/status.rs`
- Tests: `crates/lkjagent-cli/tests/status.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- Status reports context size without total window or percentage.
- Token fields are absent from a running task.
- GPT log path points to a stale or competing current file.

## Status

partially implemented
