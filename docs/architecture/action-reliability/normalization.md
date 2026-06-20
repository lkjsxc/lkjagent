# Normalization

## Purpose

This file defines deterministic action parameter normalization.

## Contract

- `doc.scaffold` and `doc.audit` rename `path` to `root` when `root` is absent.
- `workspace.summary` and `fs.list` rename `root` to `path` when `path` is absent.
- No-param graph inspection tools drop harmless location params only when the
  value is empty, `.`, `./`, `workspace`, or `/workspace`.
- Normalization emits a notice and action-normalization evidence.
- Unsafe or semantic parameters are refused, not dropped.

## Implementation Hooks

- Source: `crates/lkjagent-tools/src/dispatch/normalize.rs`
- Tests: `crates/lkjagent-tools/tests/graph_control_dispatch.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- `doc.scaffold` with `path` fails instead of normalizing to `root`.
- `graph.state` with `path=.` loops instead of producing a safe correction.
- A semantic field such as `content` is silently discarded.

## Status

partially implemented
