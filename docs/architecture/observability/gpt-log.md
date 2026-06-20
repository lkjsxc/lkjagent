# GPT Log

## Purpose

This file defines the single current Markdown handoff file that the owner can
print and send to ChatGPT or another external model.

## Contract

- Maintain `data/logs/current-gpt-5.5-pro.md`.
- Rewrite the file after significant transcript events as a synthesized snapshot.
- Treat the file as a manual export artifact, not as a monitored channel; no
  external model is expected to watch it automatically.
- Keep the current file near or below 100,000 characters, using the budget for
  useful transcript context before deterministic truncation.
- Include owner objective, constraints, state tracks, plan, touched paths,
  evidence, faults, recent transcript, and verification.
- Expose `lkjagent gpt-log` and show the path in status.
- Archive only after task closure while keeping one current file.

## Implementation Hooks

- Source: `crates/lkjagent-runtime/src/gpt_log.rs`
- Source: `crates/lkjagent-cli/src/gpt_log.rs`
- Tests: `crates/lkjagent-cli/tests/gpt_log.rs`
- Tests: `crates/lkjagent-runtime/tests/daemon_loop.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- The log is an append-only dump instead of a bounded snapshot.
- The file is documented as if ChatGPT monitors it automatically.
- Transcript context is aggressively clipped despite remaining handoff budget.
- Faults or verification gaps are omitted.
- Multiple current handoff files compete.

## Status

implemented
