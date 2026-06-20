# Verification Plan

## Purpose

This file owns the focused tests, benchmark cases, and compose smoke evidence
required before the reliability redesign can close.

## Contract

- Add focused tests for every owner-reported failure.
- Add benchmark cases for semantic docs, parameter recovery, accounting, and the GPT log.
- Run focused crate tests before workspace and compose gates.
- Report only gates that actually ran.

## Implementation Hooks

- Source: `crates/lkjagent-benchmark/src`
- Tests: `crates/lkjagent-benchmark`
- Tests: `cargo run -p lkjagent-xtask -- benchmark check-corpus`
- Verification: `docker compose run --rm verify`

## Failure Modes

- A scaffold test checks count only and misses sequence-named files.
- A recovery test asserts an error occurred but not that it was actionable.
- Compose is skipped while claiming runtime behavior is complete.

## Status

partially implemented
