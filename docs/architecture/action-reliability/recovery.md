# Recovery

## Purpose

This file owns bounded recovery after action parameter faults.

## Contract

- Parameter faults route to a specific recovery path.
- The next model-visible notice names valid tools and exact parameter names.
- The same action text is never executed twice.
- The same failing tool class cannot loop without a strategy change.
- Recovery prefers graph inspection or bounded read-only tools before waiting.

## Implementation Hooks

- Source: `crates/lkjagent-runtime/src/recovery.rs`
- Tests: `crates/lkjagent-runtime/tests/prompt_daemon.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- The daemon repeats the same invalid action three times.
- A parameter fault falls into a generic recovery state with no recipe.
- The owner is asked a question that the registry could answer.

## Status

partially implemented
