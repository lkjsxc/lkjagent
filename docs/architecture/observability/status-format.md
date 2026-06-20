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

- Source: `crates/lkjagent-cli/src/status.rs`
- Tests: `crates/lkjagent-cli/tests/commands.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- Status reports context size without total window or percentage.
- Token fields are absent from a running task.
- GPT log path is unavailable to the owner.

## Status

partially implemented
