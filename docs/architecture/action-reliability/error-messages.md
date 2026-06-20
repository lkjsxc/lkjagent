# Error Messages

## Purpose

This file defines model-visible parameter refusal text.

## Contract

- Refusals start with `action params refused`.
- Refusals include tool, expected shape, received params, hint, and valid example.
- Examples are copyable action XML with only valid fields.
- `graph.note` examples use an accepted note kind such as decision.
- Invalid `graph.note` kind refusals list every allowed value and show a
  copyable valid note action.
- Normalization notices start with `action params normalized`.
- Messages name a different tool when the emitted parameter belongs elsewhere.
- The registry prompt shows no-param tools as `no params` and gives examples
  for graph inspection and document tools.
- Recovery advice must never suggest a tool that active policy will reject.
- Large-payload parse faults suggest artifact planning or bounded section
  writes, not another raw large write.
- Owner questions are refused when the question is about internal tool use,
  valid parameters, stale memory rows, or recovery strategy.

## Implementation Hooks

- Source: `crates/lkjagent-tools/src/dispatch/validate.rs`
- Tests: `crates/lkjagent-tools/tests/graph_control_dispatch.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- The message says only `missing params` or `unknown params`.
- A valid example includes a parameter the registry rejects.
- The hint omits whether `path` or `root` is the accepted name.

## Status

partially implemented; parameter and graph-note messages are covered, but full
payload-risk and recovery-node examples remain open.
