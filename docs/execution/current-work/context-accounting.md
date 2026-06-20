# Context Accounting

## Purpose

This file owns compact context and token accounting in status, console, and
runtime persistence.

## Contract

- Show context used, total window, percentage, and pressure.
- Show input, output, cached input, and total tokens when known.
- Store unknown endpoint usage as unknown, not zero.
- Use compact decimal suffixes with two decimals for large counts.

## Implementation Hooks

- Source: `crates/lkjagent-cli/src/status.rs`
- Tests: `crates/lkjagent-cli/tests/console_render.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- Status hides context pressure during long tasks.
- Missing usage is displayed as zero.
- Console omits token accounting from the bottom deck.

## Status

not implemented
