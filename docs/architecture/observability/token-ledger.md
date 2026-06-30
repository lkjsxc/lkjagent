# Token Ledger

## Purpose

This file defines compact token usage accounting.

## Contract

- Record per-exchange input, output, cached input, total, context window, and
  used estimate.
- Unknown endpoint fields stay unknown, not zero.
- Format counts with decimal suffixes: `999`, `1.00K`, `1.23M`, `2.00B`.
- Format ratios with two decimal places.
- Status and console show context fraction and pressure.
- Aggregate APIs report latest, task, session, and all-time scopes with known
  sums plus unknown-field counts.
- Status and console render the same aggregate facts from the same deck.

## Implementation Hooks

- Source: `crates/lkjagent-llm/src/wire.rs`
- Source: `crates/lkjagent-store/src/token_usage.rs`
- Source: `crates/lkjagent-runtime/src/token_usage.rs`
- Tests: `crates/lkjagent-llm/tests/wire.rs`
- Tests: `crates/lkjagent-store/tests/token_usage.rs`
- Tests: `crates/lkjagent-runtime/tests/daemon_loop.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- Missing endpoint usage is displayed as zero.
- Cached input tokens are discarded.
- Context pressure is visible only after endpoint failure.

## Status

implemented for per-exchange usage capture, aggregate store APIs, and status
plus console rendering. Model-log output still names individual exchange usage
rather than the full aggregate deck.
