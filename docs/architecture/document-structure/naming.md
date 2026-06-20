# Naming

## Purpose

This file owns semantic names for generated documentation paths.

## Contract

- Names come from roles, not ordinal counters.
- Use kebab-case names such as `runtime-recovery.md` and `topic-map.md`.
- Duplicate roles receive semantic qualifiers such as state, recovery, or audit.
- Sequence-only names are forbidden for primary documentation children.
- Ordered reading paths belong in README content, not meaningless filenames.

## Implementation Hooks

- Source: `crates/lkjagent-tools/src/doc.rs`
- Tests: `crates/lkjagent-tools/tests/typed_tools.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- Generated docs include part-001.md, section-001.md, chapter-001.md, or equivalents.
- Disambiguation appends a bare number instead of a role.
- A generated directory name expresses ordering instead of topic.

## Status

partially implemented
