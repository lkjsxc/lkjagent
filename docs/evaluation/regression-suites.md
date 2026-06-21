# Regression Suites

## Purpose

Define the uploaded-run regression suites that keep owner-reported failures
executable.

## Suites

`authority-contradictions` covers blocked preferred actions and policy
conflicts. `artifact-readiness` covers scaffold-only and weak content roots.
`recovery-deadlocks` covers malformed actions, parameter faults, repeat
refusals, and blocked escape tools. `false-completion` covers close attempts
without evidence, audit pass, or verification.

## Fixture Names

The suite includes `cookbook_missing_evidence`,
`cookbook_scaffold_only_foundations`, `cookbook_missing_readme_links`,
`cookbook_weak_content_audit`, `tool_admission_graph_plan_contradiction`,
`parameter_fault_memory_save`, `parse_fault_unclosed_content`,
`repeat_action_refused`, `maintenance_noop_claim`, and
`false_completion_after_scaffold`. The uploaded-run matrix also includes
`artifact-readiness-graph-evidence-bypass`, which proves direct graph evidence
cannot replace artifact audit readiness.

## Verification

Run `cargo run -p lkjagent-xtask -- benchmark check-corpus`.

## Status

partially implemented.
