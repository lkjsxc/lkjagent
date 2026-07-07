# Scope

## Purpose

Define the product boundary so every feature has a clear place or no place.

## In Scope

- One daemon process executing one owner queue sequentially.
- One OpenAI-compatible chat-completions endpoint configured by the owner.
- One SQLite store under `data/` with durable cases, events, state cells,
  runtime decisions, prompt frames, admissions, observations, context items,
  artifacts, checks, memory, usage, and config.
- Arbitrary active state cells keyed by stable namespace and name.
- State-derived output grammar and tool views for each runtime decision.
- A single tool catalog with layered policy for owner settings, workspace
  bounds, active state affordances, case constraints, retry suppressors,
  evidence needs, and recovery constraints.
- Workspace file work inside the mounted repository or owner workspace.
- Plan templates as initial events and state cells for docs trees, file work,
  questions, journal entries, structured artifacts, and generic exploration.
- Durable context items with source fingerprints, trust, staleness,
  contamination class, contradiction handling, and bounded prompt admission.
- Deterministic checks for file existence, word counts, link resolution, README
  coverage, command success, artifact fingerprints, and sparse model judgment.
- CLI status, logs, case inspection, queue inspection, memory search, state and
  decision diagnostics, and a terminal watch surface.
- Docker Compose gates and proof-bundle capture.

## Out Of Scope

- Multi-user operation, remote hosting, web UI, messaging gateways, MCP, and
  runtime sub-agents.
- Permission prompts inside the daemon.
- Idle self-maintenance. With no open state requiring work and no pending queue
  item, the daemon waits.
- Dedicated personal-records tables or tools. Personal records are workspace
  files produced through normal state, tool, and check flows.
- Model-selected completion. Completion is a harness-computed state edge guarded
  by passed checks.
- Prompt-visible tools that admission will reject for the same decision.
- Transcript replay as context.

## Deletion Rule

Anything outside this scope is removed rather than hidden behind flags. Git
history is the archive; the current tree states only the active contract.
