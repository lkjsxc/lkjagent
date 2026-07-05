# Protocol Experiments

## Purpose

Define the durable experiment ledger for prompt and protocol trials.

## Record Shape

Each experiment is a generic workspace record with `kind: experiment`. The title
names the profile under test, such as `tool-call-card-current` or
`artifact-body-tags`. The body records hypothesis, prompt changes, corpus, run
commands, measured parse faults, rejected ideas, result, and next action.

## Evidence Links

Experiment records link to prompt-frame refs, provider exchanges, proof bundles,
selector candidates, and command logs. The record fingerprint is evidence; it is
not runtime authority. Selectors may later use explicit state cells derived from
experiment records, but the record file alone does not choose turns.

## Runner

`cargo run -p lkjagent-xtask -- experiment protocol --out tmp/protocol-experiment-current.md`
writes a deterministic `RuntimeDecision`-backed matrix. Rows record the decision
id, expected envelope, tool-view fingerprint, stop tag, parse result, optional
admission result, and pass or fail status. Covered cases include valid tool
calls, old action envelopes, missing or duplicate fields, unknown tools, prose
outside the block, unclosed or empty blocks, and workspace path escapes. It does
not call the endpoint.

## Trial Rule

Try combinations, not isolated tweaks, and keep rejected ideas. A protocol
profile is adopted only when docs, parser, prompt renderer, fixtures, proof, and
focused tests agree.
