# Completion Policy

## Purpose

Define the authority-owned gate that every owner-task close path must use.

## Decision Owner

`lkjagent-runtime` owns completion policy. The graph can recommend completion,
and the model can request `agent.done`, but the reducer decides close
eligibility.

## Inputs

Completion reads owner objective status, required evidence, document audit,
artifact audit, content readiness, scaffold-only paths, weak paths, line
limits, recovery state, verification output, and unsupported claims.

## Output

The gate emits allowed close, blocked close, or structured handoff. A blocked
close includes missing requirements and one admitted next valid action.

## Required Facts

Close requires resolved or explicitly blocked owner objective, present required
evidence, passing document or artifact audit when relevant, no scaffold-only
or weak leaves, no active recovery fault, observed verification result, and no
unsupported verification claim.

## Prohibited States

- `agent.done` closes while content readiness is missing.
- Failed audit is treated as a warning.
- Maintenance no-op work closes an owner objective.
- Partial completion is implied instead of explicitly recorded.

## Fixture

`false_completion_after_scaffold` proves scaffold output cannot close.
`cookbook_weak_content_audit` proves weak content leaves keep the case open.

## Verification

Run `cargo test -p lkjagent-runtime completion` and
`cargo test -p lkjagent-benchmark corpus`.

## Status

partially implemented.
