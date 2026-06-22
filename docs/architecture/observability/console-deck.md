# Console Deck

## Purpose

This file defines the compact bottom deck in the interactive console.

## Contract

- Show daemon state, queue depth, active case, top state tracks, context
  fraction, token usage, last fault, last action, pending owner question, and
  model log path.
- Keep the deck compact enough for narrow terminals.
- Unknown values render as `unknown`.
- The console remains read-only until the owner sends input.

## Implementation Hooks

- Source: `crates/lkjagent-cli/src/console/render.rs`
- Tests: `crates/lkjagent-cli/tests/console_render.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- Deck text overlaps the prompt.
- Token usage disappears when cached input is unknown.
- State tracks are hidden behind one phase label.
- Model handoff path wraps so aggressively that the filename is hidden.

## Status

partially implemented
