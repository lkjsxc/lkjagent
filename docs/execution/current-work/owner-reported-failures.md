# Owner Reported Failures

## Purpose

This file records the fresh owner-visible failures that override stale claims
until code and gates prove the behavior is repaired.

## Contract

- Treat owner reports as evidence, not as optional feedback.
- Record each confirmed failure in [../current-blockers.md](../current-blockers.md).
- Move a failure to closed only after focused tests and the agreed gate pass.

## Implementation Hooks

- Source: `crates/lkjagent-tools/src/doc.rs`
- Tests: `crates/lkjagent-tools/tests/typed_tools.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- Generated documentation uses files like part-001.md for ordinary docs.
- Action validation reports unknown params [path] without a valid copyable action.
- Status and console output omit compact context and token accounting.
- The daemon closes or waits without enough evidence for the owner task.

## Status

partially implemented
