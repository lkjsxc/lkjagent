# GPT Log

## Purpose

This file defines the single current Markdown handoff log for external model
inspection.

## Contract

- Maintain `data/logs/current-gpt-5.5-pro.md`.
- Rewrite the file after significant transcript events as a synthesized snapshot.
- Include owner objective, constraints, state tracks, plan, touched paths,
  evidence, faults, recent transcript, and verification.
- Expose `lkjagent gpt-log` and show the path in status.
- Archive only after task closure while keeping one current file.

## Implementation Hooks

- Source: `crates/lkjagent-runtime/src/gpt_log.rs`
- Tests: `crates/lkjagent-cli/tests/commands.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- The log is an append-only dump instead of a bounded snapshot.
- Faults or verification gaps are omitted.
- Multiple current handoff files compete.

## Status

not implemented
