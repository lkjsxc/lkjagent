# Partial Handoff

## Purpose

Define the structured handoff emitted when recovery cannot safely continue.

## Contract

Handoff is a blocked state, not completion. It records objective, active case,
fault class, retry count, last valid observation, last invalid action, missing
evidence, weak paths, admitted next actions, commands run, commands not run,
and the next executable step.

## When To Emit

Emit handoff after retry budget is exhausted, required external authority is
missing, a destructive branch is required, or compaction resume data is too
incomplete to continue safely.

## Invariants

- Handoff never claims owner completion.
- Handoff contains exact paths and exact next tools.
- Handoff preserves enough data for a later turn to resume or diagnose.

## Fixture

`compaction_resume_missing` proves missing resume data becomes handoff or
re-observation rather than silent restart.

## Verification

Run `cargo test -p lkjagent-runtime recovery_controller`.

## Status

partially implemented.
