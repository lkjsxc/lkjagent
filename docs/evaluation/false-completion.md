# False Completion

## Purpose

Define fixtures for premature `agent.done` and close-path mistakes.

## Contract

Known-bad traces close after scaffold generation, planning evidence,
placeholder content, failed audit, unsupported verification claims, or no-op
maintenance. Known-good traces refuse close and name the next admitted action.

## Required Cases

- `false_completion_after_scaffold`.
- `maintenance_noop_claim`.
- `artifact-readiness-graph-evidence-bypass`.
- Cookbook artifact readiness failure before close.
- Missing document audit before close.

## Pass Condition

`agent.done` is blocked until required evidence, readiness, audit, recovery,
and verification gates pass. If progress cannot safely continue, the case emits
structured handoff rather than completion.

## Verification

Run `cargo test -p lkjagent-runtime completion`.

## Status

partially implemented.
