# Scope

## Purpose

Define the product boundary so every feature has a clear place or no place.

## In Scope

- One daemon process executing one owner queue sequentially.
- One OpenAI-compatible chat-completions endpoint configured by the owner.
- One SQLite store under `data/` with durable tasks, plans, attempts, checks,
  events, memory, usage, and config.
- Workspace file work inside the mounted repository or owner workspace.
- Plan templates for manuscripts, docs trees, file work, questions, journal
  entries, and generic exploration.
- Ten explore tools for bounded discovery when a template cannot script the
  whole job.
- Deterministic checks for file existence, word counts, link resolution,
  README coverage, command success, and sparse model judgment.
- CLI status, logs, task inspection, queue inspection, memory search, and a
  terminal watch surface.
- Docker Compose gates and proof-bundle capture.

## Out Of Scope

- Multi-user operation, remote hosting, web UI, messaging gateways, MCP, and
  runtime sub-agents.
- Permission prompts inside the daemon.
- Idle self-maintenance. With no open task and no pending queue item, the daemon
  waits.
- Dedicated personal-records tables or tools. Personal records are workspace
  files written by plan templates.
- Model-selected completion. Completion is an engine state edge guarded by
  passed checks.
- A broad tool registry for scripted work. Scripted steps have fixed effects;
  model-chosen tools exist only inside explore steps.

## Deletion Rule

Anything outside this scope is removed rather than hidden behind flags. Git
history is the archive; the current tree states only the active contract.
