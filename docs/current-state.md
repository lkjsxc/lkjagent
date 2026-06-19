# Current State

## Purpose

This file is the honest ledger of lkjagent. It states what exists, what is
design-only, and what comes next. Every change that moves behavior updates
this file in the same commit. Statuses used across docs: implemented,
design-only, not implemented, out of scope, open question.

## Summary

The repository contains the documentation contract, a compiling Cargo
workspace, local verification gates, the action parser, the pure state graph,
the pure context engine, the SQLite store boundary, the LLM endpoint client,
the tool dispatcher/adapters, the runtime step/daemon core, and the CLI for
send, status, log, console, memory, startup checks, repository-root .env
loading, JSON runtime config, a /data/workspace working tree, and a resident
daemon that delivers queued owner work to the endpoint and tool dispatcher
until graph-gated agent.done. The console is transcript-first with a bottom
status/control deck and display-width wrapping for mixed English and Japanese
terminal text.

Owner messages create or resume a graph case before endpoint execution. The
graph classifies the task family, enters planning, records constraints,
risks, evidence requirements, selected context packages, and legal next
transitions, then renders a graph state notice into the first endpoint-visible
context. Runtime state persists graph cases, graph events, and graph evidence
so restart and compaction reconstruct structured state. Tool observations are
recorded as graph evidence. Completion is refused until the active graph gate
has its required evidence; a graph completion refusal names the missing kind
and points the next action to `graph.evidence`.

Recursive docs, counted structured content, and knowledge-base tasks are
graph task families that select document construction. They create
README-indexed scaffolds before endpoint work and cannot close without graph
evidence for the required document structure. File and markdown-count
requests remain deterministic control guards, including when combined with a
recursive or knowledge-base task family; exact wording is exact, while
approximate wording uses a bounded tolerance. Counted completion prefers
README-indexed roots and falls back to clean top-level output directories.
The active graph prefix renders count guards and tells the model to use one
compact `shell.run` command with direct `/bin/sh` loops and `printf`
templates for bulk creation and count verification, keep the act payload
under about 1200 characters, and avoid hardcoded `/workspace` paths, brace
expansion, cat heredocs, bash scripts, literal bodies, or one `fs.write` per
file.
For counted documentation tasks that are not recursive, knowledge-base, or
benchmark scaffolds, the daemon also writes a generic `structured-output/`
tree with the requested count before the first endpoint turn and records graph
evidence for the scaffold, then saves the task summary and closes the task.
That scaffold profiles the owner's objective by detected language and broad
deliverable kind, so design and main files carry matching headings, section
roles, objective anchors, main-range coverage, sequence ledgers,
anchor-linked body spines, and continuity handoffs while preserving the exact
count.
Runtime recovery still uses bounded notices for parse errors, repeated
actions, tool errors, endpoint max-token exits, budget exhaustion, and context
pressure; graph recovery nodes are present as the structured target for the
next runtime expansion. Endpoint outages record failed attempts, set a capped
retry deadline, and keep later polls from hitting the endpoint or appending
more error events until that deadline. The LLM client sends `</act>` as a stop
sequence and restores the stripped close tag before parsing so one endpoint
completion stays bounded to one action envelope. Length completions with a
closed act are accepted; true oversize completions record a bounded preview in
recovery.

The runtime context window defaults to 24,576 tokens, accepts 16,384 tokens as
the lower supported value, derives safe soft/hard compaction triggers from the
configured window, and uses the 2,048-token reserve as endpoint max_tokens.
Owner task turn budgets load from `task.turn-budget` and apply to new tasks,
explicit continuations after budget exhaustion, and restart summaries.
Compaction is graph-aware: it preserves the active case, phase, node, plan,
evidence, missing evidence, touched paths, selected packages, recovery
strategy, and completion guard before rebuilding the prefix. The schema has
graph memory links for later retrieval ranking, but current compaction does
not yet populate those links.

Memory remains durable retrieval, but graph cases link evidence and memories.
Empty queues open bounded graph maintenance cycles in rotation when directives
are due: distill, refine-skills, prune-memory, and audit-self. Compose wiring
is implemented.
The implementation queue is [execution/current-blockers.md](execution/current-blockers.md).

## Area Status

| Area | Status | Contract |
| --- | --- | --- |
| Documentation tree and policies | implemented | [repository/](repository/README.md) |
| Vision and scope | implemented | [vision/](vision/README.md) |
| Decision records | implemented | [decisions/](decisions/README.md) |
| Agent manual | implemented | [agent/](agent/README.md) |
| Execution queue and tasks | implemented | [execution/](execution/README.md) |
| Cargo workspace and crates | implemented | [repository/layout.md](repository/layout.md) |
| Verification xtask and quiet gates | implemented | [operations/verification.md](operations/verification.md) |
| Docker compose services | implemented | [operations/compose.md](operations/compose.md) |
| Container image skeleton | implemented | [architecture/sandbox/container.md](architecture/sandbox/container.md) |
| Daemon and agent loop | implemented | [architecture/runtime/](architecture/runtime/README.md) |
| State graph and task cases | implemented | [architecture/state-graph/](architecture/state-graph/README.md) |
| Context engine and graph-aware compaction | implemented | [architecture/context/](architecture/context/README.md) |
| Action protocol and parser | implemented | [architecture/protocol/](architecture/protocol/README.md) |
| Toolset | implemented | [architecture/tools/](architecture/tools/README.md) |
| SQLite store, transcript, and memory access | implemented | [architecture/memory/](architecture/memory/README.md) |
| LLM endpoint client | implemented | [architecture/llm/](architecture/llm/README.md) |
| Container and sandbox | implemented | [architecture/sandbox/](architecture/sandbox/README.md) |
| User message queue and CLI | implemented | [product/](product/README.md) |
| Automatic idle self-maintenance | implemented | [architecture/runtime/self-maintenance.md](architecture/runtime/self-maintenance.md) |
| Mechanical benchmark evaluation | implemented | [evaluation/](evaluation/README.md) |

## Out of Scope

Messaging channels, web UI, MCP, sub-agents, plan mode, heartbeat schedules,
and cron schedules. The boundaries are stated in [vision/scope.md](vision/scope.md).

## Next Step

No open blocker remains in
[execution/current-blockers.md](execution/current-blockers.md). Future changes
use the compose final gate per [operations/verification.md](operations/verification.md).

## Honesty Rules

- A behavior is implemented only when code, focused tests, and a passing gate exist.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
