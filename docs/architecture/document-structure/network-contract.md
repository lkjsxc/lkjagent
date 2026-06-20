# Network Contract

## Purpose

This file defines the graph layer generated beside a documentation tree.

## Contract

- Every generated documentation root includes `.lkj-doc-graph.md`.
- The graph manifest lists nodes with ids, paths, roles, and status.
- The graph manifest lists edges with source id, target id, kind, and reason.
- Coverage rows map owner requirements to concrete paths and status.
- Graph node paths point to files or directories that exist.
- Graph edge endpoints point to listed node ids.

## Implementation Hooks

- Source: `crates/lkjagent-tools/src/doc.rs`
- Tests: `crates/lkjagent-tools/tests/typed_tools.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- `.lkj-doc-graph.md` is absent after scaffold generation.
- A graph node points to a missing path.
- A graph edge points to an unknown node id.

## Status

design-only
