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
has its required evidence.

Recursive docs and knowledge-base tasks are graph task families. They create
README-indexed scaffolds before endpoint work and cannot close without graph
evidence for the required document structure. Exact markdown file counts
remain deterministic control guards that run after the graph completion gate.
Runtime recovery still uses bounded notices for parse errors, repeated
actions, tool errors, endpoint max-token exits, budget exhaustion, and context
pressure; graph recovery nodes are present as the structured target for the
next runtime expansion.

The runtime context window defaults to 24,576 tokens, accepts 16,384 tokens as
the lower supported value, derives safe soft/hard compaction triggers from the
configured window, and uses the 2,048-token reserve as endpoint max_tokens.
Compaction is graph-aware: it preserves the active case, phase, node, plan,
evidence, missing evidence, touched paths, selected packages, recovery
strategy, and completion guard before rebuilding the prefix. The schema has
graph memory links for later retrieval ranking, but current compaction does
not yet populate those links.

Memory remains durable retrieval, but graph cases link evidence and memories.
Empty queues open bounded graph maintenance cycles in rotation: distill,
improve-graph, prune-memory, and audit-self. Compose wiring is implemented.
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
