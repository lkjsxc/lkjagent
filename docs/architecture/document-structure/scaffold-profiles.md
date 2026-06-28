# Scaffold Profiles

## Purpose

Define internal semantic shape profiles used by planners and audits. These are
not live prompt-visible writer tools.

## Contract

Profiles name required root files, README coverage, local links, semantic leaf
roles, and audit expectations. Runtime decisions convert profiles into
write-contract paths; the model authors content with `fs.batch_write` line
protocol.

## Invariants

- Profiles never generate body prose for live prompts.
- Profiles never satisfy document-structure or artifact-readiness evidence by
  themselves.
- Audits own audit evidence after the model-authored writes land.

## Status

open for this redesign.
