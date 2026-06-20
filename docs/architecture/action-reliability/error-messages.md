# Error Messages

## Purpose

This file defines model-visible parameter refusal text.

## Contract

- Refusals start with `action params refused`.
- Refusals include tool, expected shape, received params, hint, and valid example.
- Examples are copyable action XML with only valid fields.
- Normalization notices start with `action params normalized`.
- Messages name a different tool when the emitted parameter belongs elsewhere.

## Implementation Hooks

- Source: `crates/lkjagent-tools/src/dispatch/validate.rs`
- Tests: `crates/lkjagent-tools/tests/graph_control_dispatch.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- The message says only `missing params` or `unknown params`.
- A valid example includes a parameter the registry rejects.
- The hint omits whether `path` or `root` is the accepted name.

## Status

partially implemented
