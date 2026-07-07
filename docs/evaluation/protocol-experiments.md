# Protocol Experiments

## Purpose

Define the durable experiment ledger for prompt and protocol trials.

## Record Shape

Each experiment is a generic workspace record with `kind: experiment`. The title
names the profile combination under test, such as `kernel-current +
tool-card-strict` or `artifact-body-tags + clean-context-small`. The body
records hypothesis, profile names, prompt changes, corpus, run commands,
measured parse faults, rejected ideas, result, and next action.

## Evidence Links

Experiment records link to prompt-frame refs, provider exchanges, proof bundles,
selector candidates, and command logs. The record fingerprint is evidence; it is
not runtime authority. Selectors may later use explicit state cells derived from
experiment records, but the record file alone does not choose turns.

## Runner

```sh
cargo run -p lkjagent-xtask -- experiment protocol \
  --profile NAME --out tmp/protocol-experiment-current.md
cargo run -p lkjagent-xtask -- experiment protocol \
  --all --out-dir tmp/protocol-experiments/<stamp>
```

The single-profile form writes one deterministic `RuntimeDecision`-backed
matrix. The `--all` form writes baseline, protocol-safe, context-kernel,
personal-workspace, software-project, artifact-manifest, and protocol-stress records
plus an `adoption.md` summary. Rows record the profile, declared feature
set, decision id, expected envelope, tool-view fingerprint, stop tag, parse
result, optional admission result, and pass or fail status. Covered cases include
valid tool calls, safe filled examples, old action envelopes, missing or
duplicate fields, unknown tools, `tool_name` ordering, placeholder values,
invalid counts, prose outside the block, unclosed or empty blocks, and workspace
path escapes. It does not call the endpoint. Profile records are evidence labels
until code explicitly wires profile-specific renderer, parser, or admission
behavior.

## Live Profiles

```sh
cargo run -p lkjagent-xtask -- experiment live-profiles \
  --out-dir tmp/live-runs/<stamp>-profiles \
  --data tmp/live-profile-data/<stamp> \
  --duration-seconds 900
```

The live runner covers personal workspace, software project, structured
artifact, and protocol stress objectives. It reads endpoint settings from
environment variables or flat `data/lkjagent.json`; if neither source has an
endpoint URL and model, it writes an honest skip summary and raw-evidence note
for every profile rather than faking endpoint success. When an endpoint is
available, each profile loop runs for the requested elapsed time unless an
endpoint or store error blocks it.

## Trial Rule

Try combinations, not isolated tweaks, and keep rejected ideas. The first
required deterministic set is baseline plus two combinations. A profile is
adopted only when docs, parser, prompt renderer, fixtures, proof, and focused
tests agree. Live endpoint trials store raw commands and summaries under
`tmp/live-runs/<stamp>/` without secrets.
