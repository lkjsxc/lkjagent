# Parameter Contract

## Purpose

This file defines the typed parameter boundary for model actions.

## Contract

- The registry is the single source for required, optional, and no-param tools.
- No-param tools render as `no params` in model-visible schemas.
- Required semantic fields are never invented by recovery.
- Optional defaults come only from the registry.
- Safe location aliases are normalized before refusal.
- Parser-level parameter faults preserve the tool name so runtime recovery can
  render the exact valid example for that tool.
- Recovery examples are generated from the registry and must remain valid after
  safe alias normalization.
- Dispatcher validation and prompt examples use the same parameter contract.

## Registry Record

```text
ToolSchema
- tool_name
- required_fields
- optional_fields
- field_encoding
- canonical_action_example
- normalization_rules
- dispatcher_parser
- admission_metadata
```

## Implementation Hooks

- Source: `crates/lkjagent-protocol/src/parse.rs`
- Source: `crates/lkjagent-protocol/src/model.rs`
- Source: `crates/lkjagent-tools/src/dispatch/validate.rs`
- Source: `crates/lkjagent-tools/src/dispatch/examples.rs`
- Source: `crates/lkjagent-runtime/src/recovery.rs`
- Tests: `crates/lkjagent-protocol/tests/fixtures.rs`
- Tests: `crates/lkjagent-runtime/tests/fault_wait.rs`
- Tests: `crates/lkjagent-tools/tests/graph_control_dispatch.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- A no-param tool accepts arbitrary semantic content.
- A missing required summary, objective, or question is invented.
- The model sees only `unknown params` and no valid shape.
- A generated valid example is later rejected by dispatcher validation.

## Status

partially implemented
