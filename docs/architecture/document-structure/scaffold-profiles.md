# Scaffold Profiles

## Purpose

This file defines deterministic semantic shapes for `doc.scaffold`.

## Contract

- Profile selection uses normalized objective, kind, sections, count, and mode.
- General project docs use overview, architecture, guides, operations, and reference.
- Knowledge bases use concepts, maps, notes, synthesis, and reference.
- Implementation plans use diagnosis, design, tasks, and acceptance.
- Reports use summary, analysis, recommendations, and appendices.
- Manuscript-like outputs still use semantic chapter-arc names, not bare ordinals.

## Implementation Hooks

- Source: `crates/lkjagent-tools/src/doc.rs`
- Tests: `crates/lkjagent-tools/tests/typed_tools.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- Profile selection copies the raw owner message as the structure.
- A requested documentation tree falls through to sequence-only files.
- Count fitting deletes README indexes or the graph manifest.

## Status

design-only
