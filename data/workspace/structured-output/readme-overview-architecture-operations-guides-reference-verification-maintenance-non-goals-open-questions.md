# README,Overview,Architecture,Operations,Guides,Reference,Verification,Maintenance,Non Goals,Open Questions

## Purpose

This file records the README,Overview,Architecture,Operations,Guides,Reference,Verification,Maintenance,Non-Goals,Open Questions role for the generated documentation tree.

## Contract

- Keep this file semantic and linked from its local README.
- Record concrete facts, decisions, and verification evidence.

## Implementation Hooks

- Source: `crates/lkjagent-tools/src/doc.rs`
- Tests: `crates/lkjagent-tools/tests/typed_tools.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- The file is unlinked from its directory README.
- The file becomes a placeholder without role-specific content.

## Status

scaffolded
