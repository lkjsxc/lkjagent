# Completion Gates

## Purpose

This file defines the evidence required before documentation work can close.

## Contract

- Completion requires a passing `doc.audit` for the generated root.
- Completion requires document-structure graph evidence on the active case.
- Completion is illegal while sequence-only paths, missing README files, or
  missing catalog metadata checks remain open.
- Completion summaries name the generated root and verification command.
- Content artifacts require README, manifest, semantic children,
  content-bearing files, and a passing audit.
- Planning-only or scaffold-only output cannot close an artifact task.
- Artifact completion also obeys
  [../artifacts/completion-gates.md](../artifacts/completion-gates.md).

## Implementation Hooks

- Source: `crates/lkjagent-graph/src/completion.rs`
- Tests: `crates/lkjagent-runtime/tests/recursive_scaffold.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- `agent.done` closes after scaffold creation but before audit evidence.
- Completion evidence names count only and omits topology.
- A failed audit is recorded as success.

## Status

partially implemented; audit and scaffold-only refusal exist. Full artifact
completion readiness remains open.
