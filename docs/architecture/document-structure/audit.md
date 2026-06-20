# Audit

## Purpose

This file owns the deterministic document topology audit.

## Contract

- Audit checks the root exists and has `README.md`.
- Audit checks each directory has one `README.md` and at least two children
  unless explicitly terminal.
- Audit checks README purpose sections and local child links.
- Audit rejects sequence-only documentation names.
- Audit rejects release-number API shorthand unless a local exception table
  explains why the path is literal source material.
- Audit checks one H1 per Markdown file, line caps, graph nodes, and graph edges.
- Audit output is compact, lists failed check names, and names exact next actions.

## Implementation Hooks

- Source: `crates/lkjagent-tools/src/doc.rs`
- Tests: `crates/lkjagent-tools/tests/typed_tools.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- Audit passes a README that omits local children.
- Audit passes a missing `.lkj-doc-graph.md`.
- Audit reports only generic failure text without repair targets.

## Status

partially implemented
