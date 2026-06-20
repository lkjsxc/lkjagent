# Tree Contract

## Purpose

This file defines the recursive filesystem shape for generated documentation.

## Contract

- Documentation roots contain `README.md` and at least two semantic children.
- Every directory contains exactly one `README.md`.
- Every README links all local Markdown files and child directories.
- README files expose purpose, local map, reading paths, and cross-links.
- Leaf documents state contract, hooks, failure modes, and status.
- A documentation tree is invalid if primary child files are named only by
  sequence and lack semantic roles.

## Implementation Hooks

- Source: `crates/lkjagent-tools/src/doc.rs`
- Tests: `crates/lkjagent-tools/tests/typed_tools.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- A directory lacks `README.md`.
- A README omits a local child link.
- A primary child file is only an ordinal placeholder.

## Status

partially implemented
