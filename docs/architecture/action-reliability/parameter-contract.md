# Parameter Contract

## Purpose

This file defines the typed parameter boundary for model actions.

## Contract

- The registry is the single source for required, optional, and no-param tools.
- No-param tools render as `no params` in model-visible schemas.
- Required semantic fields are never invented by recovery.
- Optional defaults come only from the registry.
- Safe location aliases are normalized before refusal.

## Implementation Hooks

- Source: `crates/lkjagent-tools/src/dispatch/validate.rs`
- Tests: `crates/lkjagent-tools/tests/graph_control_dispatch.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- A no-param tool accepts arbitrary semantic content.
- A missing required summary, objective, or question is invented.
- The model sees only `unknown params` and no valid shape.

## Status

partially implemented
