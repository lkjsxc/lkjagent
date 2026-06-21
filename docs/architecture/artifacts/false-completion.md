# False Completion

## Purpose

Define artifact close refusals for scaffold-only or weak content cases.

## Contract

Artifact completion is blocked when any required content leaf is absent, weak,
scaffold-only, unlinked from a local README, over the line limit, mismatched
to the profile, or backed only by planning evidence.

## Refusal Output

A refusal names blocked gates, weak paths, missing links, missing sections,
last audit result, admitted next action, and whether structured handoff is
required.

## Invariants

- Failed audit keeps the active case open.
- `agent.done` refusal uses the same gate as runtime close.
- Scaffolding is structure evidence only.
- Verification notes cannot claim content that is absent.

## Fixture

`false_completion_after_scaffold` and `cookbook_scaffold_false_ready` prove a
large cookbook scaffold cannot close until content readiness and audit pass.

## Verification

Run `cargo test -p lkjagent-runtime artifact_completion_gate` and
`cargo test -p lkjagent-benchmark corpus`.

## Status

partially implemented.
