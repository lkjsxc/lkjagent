# Model Log

## Purpose

This file defines the single current Markdown handoff file that the owner can
print and send to a selected external model. Raw per-call model input and
output records live in [provider-exchange-log.md](provider-exchange-log.md).

## Contract

- Maintain `data/logs/current-model-run.md`.
- Rewrite the file after significant transcript events as a synthesized snapshot.
- Treat the file as a manual export artifact, not as a monitored channel; no
  external model is expected to watch it automatically.
- Keep the current file near or below 1,000,000 characters, using the budget for
  useful transcript context before deterministic truncation.
- Include owner objective, constraints, state tracks, plan, touched paths,
  evidence, faults, recent transcript, and verification.
- Expose `lkjagent model-log` and show the path in status.
- Archive only after task closure while keeping one current file.
- Do not use this file as the only replay record for provider requests and responses.

## Implementation Hooks

- Source: `crates/lkjagent-runtime/src/model_log.rs`
- Source: `crates/lkjagent-cli/src/model_log.rs`
- Tests: `crates/lkjagent-cli/tests/model_log.rs`
- Tests: `crates/lkjagent-runtime/tests/daemon_loop.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- The log is an append-only dump instead of a bounded snapshot.
- The file is documented as if an external model monitors it automatically.
- Transcript context is aggressively clipped despite remaining handoff budget.
- Faults or verification gaps are omitted.
- Multiple current handoff files compete.

## Status

implemented
